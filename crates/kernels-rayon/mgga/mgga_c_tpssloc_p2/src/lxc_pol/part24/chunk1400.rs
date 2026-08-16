//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1400/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1400(t23518: f64, t23634: f64, t1003: f64, t1022: f64, t10359: f64, t1058: f64, t1060: f64, t11037: f64, t1953: f64, t23571: f64, t23604: f64, t23609: f64, t23633: f64, t23635: f64, t23657: f64, t23658: f64, t23662: f64, t23670: f64, t23678: f64, t23707: f64, t3076: f64, t3120: f64, t353: f64, t383: f64, t43240: f64, t607: f64, t6797: f64, t6800: f64, t6813: f64, t83226: f64, t83233: f64, t83234: f64, t83239: f64, t83240: f64, t83245: f64, t83246: f64, t83247: f64) -> f64 {
    let t83265 = t23518 * t23634;
    let t83270 = -3.0_f64 * t11037 * t23662 + 3.0_f64 * t1058 * t23571 * t1022 * t1060 + 3.0_f64 * t1003 * t23707 + t353 * t383 * t83226 + 0.82246703342411321826e-2_f64 * t23633 * t23635 * t43240 * t6800 - 0.16449340668482264365e-1_f64 * t23633 * t83233 * t83234 + 0.10966227112321509577e-1_f64 * t83239 * t83240 * t83234 + 0.16449340668482264365e-1_f64 * t83245 * t83246 * t83247 * t23678 + 0.82246703342411321826e-2_f64 * t23633 * t23635 * t607 * t3120 * t6800 + t10359 * t1953 + 3.0_f64 * t3076 * t6813 + 0.13159472534785811492e0_f64 * t23670 * t23658 - 0.49348022005446793095e-1_f64 * t6797 * t23657 * t23609 - 0.82246703342411321826e-2_f64 * t83245 * t83265 * t83247 * t23604;
    t83270
}
