//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 461/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk461(t1335: f64, t3862: f64, t3861: f64, t3793: f64, t453: f64, t3781: f64, t1324: f64, t3809: f64, t1060: f64, t250: f64, t461: f64, t1331: f64, t659: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3863 = t3862 * t1335;
    let t3865 = 2.0_f64 * t3861 * t3863;
    let t3868 = 0.39862222222222222223e0_f64 * t3793;
    let t3873 = 1.0_f64/f64::sqrt(t453);
    let t3874 = t3873 * t3781;
    let t3876 = t1324 * t3809;
    let t3879 = t250 * t1060 * t461;
    let t3880 = 0.13692777777777777778e0_f64 * t3879;
    let t3881 = t659 * t1331;
    (t3863, t3865, t3868, t3873, t3874, t3876, t3879, t3880, t3881)
}
