//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1098/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1098<F: Float>(t866: F, t9530: F, t41960: F, t1356: F, t27102: F, t36756: F, t36758: F, t36804: F, t36806: F, t36809: F, t36811: F, t36814: F, t38079: F, t38080: F, t4044: F, t41949: F, t41954: F, t41956: F, t41958: F, t5181: F, t699: F, t739: F, t8041: F) -> (F, F) {
    let t43925 = t9530 * t866;
    let t43937 = F::new(0.11918087970123395032e-3) * t41960;
    let t43944 = F::new(0.12195059916630011325e-2) * t36756 + F::new(0.1921128438866447784e-2) * t36758 + F::new(0.39914139006212695214e-1) * t1356 * t43925 - t38079 + t38080 + F::new(0.325201597776800302e-2) * t36804 + F::new(0.3842256877732895568e-2) * t36806 + F::new(0.325201597776800302e-2) * t36809 + F::new(0.3842256877732895568e-2) * t36811 - F::new(0.30487649791575028312e-3) * t36814 + F::new(0.20431007948782962912e-3) * t41949 + F::new(0.5107751987195740728e-4) * t41954 - F::new(0.5107751987195740728e-4) * t41956 + F::new(0.212822999466489197e-4) * t41958 - t43937 - F::new(0.71845450211182851384e0) * t4044 * t699 * t5181 - F::new(0.35922725105591425692e0) * t739 * t8041 * t27102;
    (t43925, t43944)
}
