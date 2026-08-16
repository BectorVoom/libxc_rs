//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1498;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1499;
use chunk2::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1500;
use chunk3::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1501;
use chunk4::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1502;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta397<F: Float>(t17271: F, t894: F, t901: F, t17157: F, t2826: F, t136: F, t5717: F, t699: F, t5720: F, t5723: F, t17187: F, t908: F, t13598: F, t13712: F, t17149: F, t17165: F, t17175: F, t17189: F, t13642: F, t13709: F, t17154: F, t17159: F, t17163: F, t17169: F, t17211: F, t17213: F, t17216: F, t17219: F, t17221: F, t17224: F, t17238: F, t17241: F, t17244: F, t17247: F, t17250: F, t17253: F, t17256: F, t942: F, t951: F, t959: F, t2940: F, t5812: F, t5811: F, t952: F, t10296: F, t10556: F, t10784: F, t10785: F, t13552: F, t13566: F, t14287: F, t14291: F, t17173: F, t17180: F, t17185: F, t14324: F, t14321: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17272, t17274, t17280, t17286, t17288, t17290, t17292) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1498::<F>(t17271, t894, t901, t17157, t2826, t136, t5717, t699, t5720, t5723, t17187, t908);
        let (t17293, t17295) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1499::<F>(t136, t17292, t13598, t13712, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290);
        let t17297 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1500::<F>(t13642, t13709, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17238, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17295);
        let (t17301, t17303, t17306, t17325) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1501::<F>(t17297, t942, t951, t959, t2940, t5812, t5811, t952, t10296, t10556, t10784, t10785, t13552, t13566, t14287, t14291, t17173, t17180, t17185);
        let t17349 = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1502::<F>(t13598, t14324, t17149, t17165, t17175, t17189, t17280, t17286, t17288, t17290, t17293, t13642, t14321, t17154, t17159, t17163, t17169, t17211, t17213, t17216, t17219, t17221, t17224, t17241, t17244, t17247, t17250, t17253, t17256, t17272, t17274, t17325);
    (t17272, t17274, t17280, t17286, t17288, t17290, t17293, t17297, t17301, t17303, t17306, t17349)
}
