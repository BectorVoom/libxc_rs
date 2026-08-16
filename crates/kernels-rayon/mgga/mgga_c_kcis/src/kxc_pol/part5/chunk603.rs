//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 603/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk603(t3820: f64, t513: f64, t1317: f64, t1416: f64, t3793: f64, t1311: f64, t1315: f64, t1314: f64, t465: f64, t455: f64, t453: f64, t1060: f64, t250: f64, t461: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3821 = t3820 * t513;
    let t3824 = t1317 * t1416;
    let t3833 = 0.55033333333333333333e-2_f64 * t3793;
    let t3848 = 0.23744444444444444444e-1_f64 * t3793;
    let t3856 = t1311 * t1315;
    let t3859 = t1314 * t465;
    let t3860 = 1.0_f64 / t3859;
    let t3861 = t455 * t3860;
    let t3868 = 0.39862222222222222223e0_f64 * t3793;
    let t3873 = 1.0_f64/f64::sqrt(t453);
    let t3879 = t250 * t1060 * t461;
    (t3821, t3824, t3833, t3848, t3856, t3860, t3861, t3868, t3873, t3879)
}
