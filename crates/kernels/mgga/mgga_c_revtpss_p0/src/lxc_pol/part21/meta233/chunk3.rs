//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1381/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1381<F: Float>(t1399: F, t5704: F, t3936: F, t1370: F, t3934: F, t3944: F, t3950: F, t3953: F, t3958: F, t3967: F, t3976: F, t3982: F, t3987: F, t3990: F, t3996: F, t5681: F, t5686: F, t5690: F, t5697: F, t5701: F) -> (F, F) {
    let t5705 = t5704 * t1399;
    let t5706 = t3936 * t5705;
    let t5709 = F::new(7.0) / F::new(144.0) * t5681 + F::cast_from(0.28582678745379824648e-4_f64) * t3953 - t3976 + t3987 + F::new(7.0) / F::new(144.0) * t3958 + t3944 * t5686 / F::new(16.0) + t3967 - t1370 * t5690 / F::new(48.0) - F::cast_from(0.50820002809285328224e-4_f64) * t3982 + F::cast_from(0.40015750243531754508e-2_f64) * t3990 + F::cast_from(0.71456696863449561619e-5_f64) * t3996 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t5697 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t5701 + F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t5706 + t3950;
    (t5706, t5709)
}
