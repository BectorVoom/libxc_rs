//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 329/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk329<F: Float>(t51: F, t1226: F, t1228: F, t476: F, t1223: F, zeta_threshold: F) -> (F,) {
    let t52 = t51 <= zeta_threshold;
    let t1232 = piecewise3(t52, 0.0, -2.0 / 9.0 * t1226 + 2.0 / 3.0 * t476 * t1228);
    let t1234 = t1223 / 2.0 + t1232 / 2.0;
    (t1234,)
}
