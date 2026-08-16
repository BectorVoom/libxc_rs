//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1024/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1024<F: Float>(t112535: F, t112537: F, t112542: F, t115208: F, t115210: F, t115212: F, t115217: F, t115227: F, t115229: F, t115231: F, t115233: F, t12823: F, t2036: F, t2314: F, t23909: F, t24924: F, t24932: F, t27888: F, t32318: F, t32365: F, t4034: F, t7050: F, t7057: F, t7266: F, t8835: F) -> F {
    let t117567 = -F::cast_from(2.0_f64) * t12823 * t8835 - t2036 * t24924 - F::cast_from(4.0_f64) * t2314 * t32365 - F::cast_from(2.0_f64) * t23909 * t7266 - F::cast_from(4.0_f64) * t24932 * t7050 - F::cast_from(4.0_f64) * t27888 * t7057 - F::cast_from(4.0_f64) * t32318 * t4034 - t112535 - t112537 - t112542 - t115208 - t115210 - t115212 - t115217 + t115227 - t115229 - t115231 - t115233;
    t117567
}
