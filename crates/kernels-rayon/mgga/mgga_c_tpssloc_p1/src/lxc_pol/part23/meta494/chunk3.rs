//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1522/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1522(t6439: f64, t12021: f64, t1375: f64, t1807: f64, t1843: f64, t20044: f64, t20060: f64, t20601: f64, t20609: f64, t20662: f64, t40591: f64, t5215: f64, t5321: f64, t539: f64, t568: f64, t6440: f64, t6460: f64, t6461: f64, t74860: f64, t74908: f64, t80477: f64) -> f64 {
    let t80511 = t6439 * t6439;
    let t80521 = -36.0_f64 * t12021 * t1375 * t6439 * t6460 + 24.0_f64 * t1375 * t40591 * t80511 + 4.0_f64 * t1807 * t20601 * t568 + t539 * t568 * t80477 - 12.0_f64 * t1843 * t74860 - 12.0_f64 * t1843 * t74908 + 12.0_f64 * t20044 * t6440 - 6.0_f64 * t20044 * t6461 + 12.0_f64 * t20060 * t6440 - 24.0_f64 * t20609 * t5215 - 24.0_f64 * t20609 * t5321 - 4.0_f64 * t20662 * t5215 - 4.0_f64 * t20662 * t5321;
    t80521
}
