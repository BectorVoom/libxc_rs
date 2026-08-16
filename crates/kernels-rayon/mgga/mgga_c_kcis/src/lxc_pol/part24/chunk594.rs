//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 594/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk594(t2943: f64, t6320: f64, t6338: f64, t932: f64, t3088: f64, t4612: f64, t6328: f64, t6332: f64, t6336: f64, t1036: f64, t1670: f64, t245: f64, t3078: f64, t4654: f64) -> (f64, f64, f64, f64) {
    let t6341 = t2943 * t6320;
    let t6343 = t932 * t6338;
    let t6349 = -0.991e-2_f64 * t6341 + 0.1982e-1_f64 * t6343 + t3088 + 0.27516666666666666666e-2_f64 * t4612 - 0.27516666666666666667e-2_f64 * t6328 + 0.8255e-2_f64 * t6332 - 0.41275e-2_f64 * t6336;
    let t6352 = -t3078 * t6320 / 8.0_f64 + t4654 * t1670 / 2.0_f64 + t1036 * t6338 / 4.0_f64 + t245 * t6349 / 2.0_f64;
    (t6341, t6343, t6349, t6352)
}
