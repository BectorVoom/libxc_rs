//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 898/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk898(t1340: f64, t13407: f64, t1411: f64, t13367: f64, t13372: f64, t13375: f64, t13380: f64, t13385: f64, t13387: f64, t13389: f64, t13392: f64, t13397: f64, t13400: f64, t13404: f64) -> (f64, f64) {
    let t13408 = t1340 * t13407;
    let t13409 = t1411 * t13408;
    let t13411 = -0.2653111111111111111e-1_f64 * t13367 + 0.16581944444444444444e-2_f64 * t13372 + 0.49745833333333333332e-2_f64 * t13375 - 0.66327777777777777776e-2_f64 * t13380 - 0.74618749999999999998e-2_f64 * t13385 + 0.99491666666666666664e-2_f64 * t13387 + 0.2653111111111111111e-1_f64 * t13389 - 0.16581944444444444444e-2_f64 * t13392 - 0.16581944444444444444e-2_f64 * t13397 + t13400 - 0.72960555555555555553e-1_f64 * t13404 + 0.48640370370370370369e-1_f64 * t13409;
    (t13409, t13411)
}
