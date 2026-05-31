//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 871/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk871<F: Float>(t2055: F, t2056: F, t955: F, t2768: F, t761: F, t2061: F, t6030: F, t6033: F, t7108: F, t7110: F, t7112: F, t7126: F, t7128: F, t7149: F, t7150: F) -> F {
    let t7898 = t2055 * t955 * t2056;
    let t7902 = t2768 * t761;
    let t7904 = F::cast_from(0.1350520664e0_f64) * t2061 * t7902;
    let t7905 = t7108 - t7110 - t7112 - F::cast_from(0.571528e-1_f64) * t7898 + F::cast_from(0.2701041328e0_f64) * t6030 - F::cast_from(0.675260332e-1_f64) * t6033 - t7126 - t7128 - t7904 + t7149 + t7150;
    t7905
}
