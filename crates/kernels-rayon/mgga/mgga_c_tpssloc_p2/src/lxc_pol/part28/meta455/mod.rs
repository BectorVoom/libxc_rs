//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1653;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1654;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta455(t22990: f64, t23000: f64, t23002: f64, t23006: f64, t23022: f64, t23026: f64, t23028: f64, t23038: f64, t24246: f64, t24250: f64, t24251: f64, t24256: f64, t2617: f64, t7102: f64, t812: f64, t23173: f64, t7084: f64, t814: f64, t829: f64, t2679: f64, t7101: f64, t235: f64, t24234: f64, t2051: f64, t226: f64, t23156: f64, t23160: f64, t23166: f64, t23169: f64, t23178: f64, t23182: f64, t23187: f64, t2613: f64, t7104: f64, t808: f64, t858: f64, t23230: f64, t225: f64, t7072: f64, t23198: f64, t23206: f64, t23209: f64, t23220: f64, t23224: f64, t23232: f64, t23235: f64, t23239: f64, t24200: f64, t24235: f64, t24237: f64, t259: f64, t2597: f64, t2713: f64, t2720: f64, t7087: f64, t7092: f64, t7107: f64, t855: f64, t866: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t24260 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1653(t22990, t23000, t23002, t23006, t23022, t23026, t23028, t23038, t24246, t24250, t24251, t24256, t2617, t7102, t812);
        let (t24265, t24269, t24270, t24273, t24278, t24280) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1654(t23173, t7084, t814, t829, t2679, t7101, t235, t24234, t2051, t226, t23156, t23160, t23166, t23169, t23178, t23182, t23187, t2613, t7104, t808, t812);
        let (t24281, t24282, t24291, t24297, t24300) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1655(t24260, t24280, t858, t23230, t225, t7072, t23198, t23206, t23209, t23220, t23224, t23232, t23235, t23239, t24200, t24235, t24237, t259, t2597, t2713, t2720, t7087, t7092, t7107, t855, t866);
    (t24265, t24269, t24270, t24273, t24278, t24281, t24282, t24291, t24297, t24300)
}
