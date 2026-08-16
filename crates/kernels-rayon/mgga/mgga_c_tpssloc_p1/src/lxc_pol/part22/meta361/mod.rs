//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1598;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1599;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1600;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1601;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1602;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1603;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta361(t10556: f64, t10577: f64, t13598: f64, t13600: f64, t13601: f64, t13603: f64, t17149: f64, t17154: f64, t17159: f64, t17163: f64, t17165: f64, t17169: f64, t17173: f64, t17175: f64, t17180: f64, t17185: f64, t17189: f64, t894: f64, t901: f64, t17157: f64, t2826: f64, t136: f64, t5717: f64, t699: f64, t5720: f64, t5723: f64, t17187: f64, t908: f64, t13712: f64, t13642: f64, t13709: f64, t17211: f64, t17213: f64, t17216: f64, t17219: f64, t17221: f64, t17224: f64, t17238: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t17271 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1598(t10556, t10577, t13598, t13600, t13601, t13603, t17149, t17154, t17159, t17163, t17165, t17169, t17173, t17175, t17180, t17185, t17189);
        let (t17272, t17274, t17279, t17280, t17286) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1599(t17271, t894, t901, t17157, t2826, t136, t5717, t699);
        let t17288 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1600(t5720, t699);
        let t17290 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1601(t5723, t699);
        let (t17292, t17293, t17295) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1602(t17187, t908, t136, t13598, t13712, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290);
        let t17297 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1603(t13642, t13709, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17238, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17295);
    (t17271, t17272, t17274, t17279, t17280, t17286, t17288, t17290, t17292, t17293, t17297)
}
