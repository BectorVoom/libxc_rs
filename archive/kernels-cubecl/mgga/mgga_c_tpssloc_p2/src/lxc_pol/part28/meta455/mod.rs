//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta455 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1653;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1654;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1655;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta455<F: Float>(t22990: F, t23000: F, t23002: F, t23006: F, t23022: F, t23026: F, t23028: F, t23038: F, t24246: F, t24250: F, t24251: F, t24256: F, t2617: F, t7102: F, t812: F, t23173: F, t7084: F, t814: F, t829: F, t2679: F, t7101: F, t235: F, t24234: F, t2051: F, t226: F, t23156: F, t23160: F, t23166: F, t23169: F, t23178: F, t23182: F, t23187: F, t2613: F, t7104: F, t808: F, t858: F, t23230: F, t225: F, t7072: F, t23198: F, t23206: F, t23209: F, t23220: F, t23224: F, t23232: F, t23235: F, t23239: F, t24200: F, t24235: F, t24237: F, t259: F, t2597: F, t2713: F, t2720: F, t7087: F, t7092: F, t7107: F, t855: F, t866: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t24260 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1653::<F>(t22990, t23000, t23002, t23006, t23022, t23026, t23028, t23038, t24246, t24250, t24251, t24256, t2617, t7102, t812);
        let (t24265, t24269, t24270, t24273, t24278, t24280) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1654::<F>(t23173, t7084, t814, t829, t2679, t7101, t235, t24234, t2051, t226, t23156, t23160, t23166, t23169, t23178, t23182, t23187, t2613, t7104, t808, t812);
        let (t24281, t24282, t24291, t24297, t24300) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1655::<F>(t24260, t24280, t858, t23230, t225, t7072, t23198, t23206, t23209, t23220, t23224, t23232, t23235, t23239, t24200, t24235, t24237, t259, t2597, t2713, t2720, t7087, t7092, t7107, t855, t866);
    (t24265, t24269, t24270, t24273, t24278, t24281, t24282, t24291, t24297, t24300)
}
