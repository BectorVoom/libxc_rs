//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1178/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1178(t2727: f64, t206: f64, t220: f64, t8942: f64, t870: f64, t8943: f64, t687: f64, t8747: f64, t2388: f64, t2379: f64, t2385: f64, t60: f64, t81: f64, t9260: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t36901 = t2727 * t2727;
    let t36902 = 1.0_f64 / t36901;
    let t36908 = t206 / t8942 / t220;
    let t36936 = t870 * t8943;
    let t36951 = t8747 * t687;
    let t36957 = t2388 * t2388;
    let t36958 = 1.0_f64 / t36957;
    let t36962 = t2379 * t2385;
    let t37000 = t60 / t9260 / t81;
    (t36902, t36908, t36936, t36951, t36958, t36962, t37000)
}
