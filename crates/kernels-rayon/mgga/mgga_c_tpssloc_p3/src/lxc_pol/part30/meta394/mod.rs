//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta394 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1499;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1500;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1501;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1502;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1503;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta394(t17271: f64, t894: f64, t901: f64, t17157: f64, t2826: f64, t136: f64, t5717: f64, t699: f64, t5720: f64, t5723: f64, t17187: f64, t908: f64, t13598: f64, t13712: f64, t17149: f64, t17165: f64, t17175: f64, t17189: f64, t13642: f64, t13709: f64, t17154: f64, t17159: f64, t17163: f64, t17169: f64, t17211: f64, t17213: f64, t17216: f64, t17219: f64, t17221: f64, t17224: f64, t17238: f64, t17241: f64, t17244: f64, t17247: f64, t17250: f64, t17253: f64, t17256: f64, t942: f64, t951: f64, t959: f64, t2940: f64, t5812: f64, t5811: f64, t952: f64, t10296: f64, t10556: f64, t10784: f64, t10785: f64, t13552: f64, t13566: f64, t14287: f64, t14291: f64, t17173: f64, t17180: f64, t17185: f64, t14324: f64, t14321: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t17272, t17274, t17280, t17286, t17288, t17290, t17292) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1499(t17271, t894, t901, t17157, t2826, t136, t5717, t699, t5720, t5723, t17187, t908);
        let (t17293, t17295) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1500(t136, t17292, t13598, t13712, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290);
        let t17297 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1501(t13642, t13709, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17238, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17295);
        let (t17301, t17303, t17306, t17325) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1502(t17297, t942, t951, t959, t2940, t5812, t5811, t952, t10296, t10556, t10784, t10785, t13552, t13566, t14287, t14291, t17173, t17180, t17185);
        let t17349 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1503(t13598, t14324, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t14321, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17325);
    (t17272, t17274, t17280, t17286, t17288, t17290, t17293, t17297, t17301, t17303, t17306, t17349)
}
