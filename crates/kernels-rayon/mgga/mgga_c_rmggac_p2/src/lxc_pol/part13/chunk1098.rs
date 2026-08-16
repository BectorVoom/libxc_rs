//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1098/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1098(t866: f64, t9530: f64, t41960: f64, t1356: f64, t27102: f64, t36756: f64, t36758: f64, t36804: f64, t36806: f64, t36809: f64, t36811: f64, t36814: f64, t38079: f64, t38080: f64, t4044: f64, t41949: f64, t41954: f64, t41956: f64, t41958: f64, t5181: f64, t699: f64, t739: f64, t8041: f64) -> (f64, f64) {
    let t43925 = t9530 * t866;
    let t43937 = 0.11918087970123395032e-3_f64 * t41960;
    let t43944 = 0.12195059916630011325e-2_f64 * t36756 + 0.1921128438866447784e-2_f64 * t36758 + 0.39914139006212695214e-1_f64 * t1356 * t43925 - t38079 + t38080 + 0.325201597776800302e-2_f64 * t36804 + 0.3842256877732895568e-2_f64 * t36806 + 0.325201597776800302e-2_f64 * t36809 + 0.3842256877732895568e-2_f64 * t36811 - 0.30487649791575028312e-3_f64 * t36814 + 0.20431007948782962912e-3_f64 * t41949 + 0.5107751987195740728e-4_f64 * t41954 - 0.5107751987195740728e-4_f64 * t41956 + 0.212822999466489197e-4_f64 * t41958 - t43937 - 0.71845450211182851384e0_f64 * t4044 * t699 * t5181 - 0.35922725105591425692e0_f64 * t739 * t8041 * t27102;
    (t43925, t43944)
}
