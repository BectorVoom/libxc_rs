//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta198 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk798;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk799;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk800;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta198<F: Float>(t1248: F, t471: F, t5332: F, t3720: F, t1222: F, t1235: F, t1238: F, t1252: F, t1261: F, t1791: F, t3637: F, t3667: F, t3711: F, t5293: F, t5299: F, t5304: F, t5309: F, t5313: F, t5320: F, t5323: F, t5327: F, t5331: F, t3767: F, t5330: F, t3603: F, t1774: F, t1250: F, t1794: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5333, t5334, t5335, t5338) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk798::<F>(t1248, t471, t5332, t3720, t1222, t1235, t1238, t1252, t1261, t1791, t3637, t3667, t3711, t5293, t5299, t5304, t5309, t5313, t5320, t5323, t5327, t5331);
        let t5340 = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk799::<F>(t3767, t5330);
        let (t5341, t5342, t5343, t5346, t5347, t5348, t5351) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk800::<F>(t1248, t3603, t5332, t3720, t1774, t1250, t1794, t73);
    (t5333, t5334, t5335, t5338, t5340, t5341, t5342, t5343, t5346, t5347, t5348, t5351)
}
