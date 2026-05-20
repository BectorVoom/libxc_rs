//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta246 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk934;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk935;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk936;
use chunk3::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta246<F: Float>(t3699: F, t5819: F, t1012: F, t1225: F, t5825: F, t3692: F, t344: F, t5843: F, t3618: F, t6421: F, t247: F, t1264: F, t6429: F, t6425: F, t1774: F, t1794: F, t1250: F, t3720: F, t1222: F, t1261: F, t1782: F, t1808: F, t3657: F, t3684: F, t3718: F, t464: F, t5358: F, t5363: F, t5366: F, t5373: F, t5379: F, t5381: F, t5391: F, t6651: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6652, t6653, t6658, t6659, t6662, t6663, t6667, t6673, t6678) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk934::<F>(t3699, t5819, t1012, t1225, t5825, t3692, t344, t5843, t3618, t6421, t247, t1264, t6429);
        let (t6679, t6683, t6688) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk935::<F>(t247, t6678, t1264, t6425, t1774, t1794);
        let (t6689, t6690, t6694) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk936::<F>(t1250, t6688, t3720, t1222, t1261, t1782, t1808, t3657, t3684, t3718, t464, t5358, t5363, t5366, t5373, t5379, t5381, t5391, t6653, t6659, t6663, t6667, t6673, t6679, t6683);
        let t6695 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk937::<F>(t6651, t6694);
    (t6652, t6658, t6662, t6667, t6673, t6679, t6683, t6688, t6689, t6690, t6695)
}
