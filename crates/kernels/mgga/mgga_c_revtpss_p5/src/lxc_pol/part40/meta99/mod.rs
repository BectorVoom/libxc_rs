//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta99 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk543;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk544;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk545;
use chunk3::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk546;
use chunk4::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk547;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta99<F: Float>(t2339: F, t2340: F, t613: F, t99: F, t658: F, t100: F, t2256: F, t107: F, tau0: F, t661: F, t108: F, t101: F, t105: F, t656: F, t659: F, t97: F, t114: F, t655: F, t2335: F, t2336: F, t69: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2341, t2344, t2349, t2350, t2351, t2354, t2357) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk543::<F>(t2339, t2340, t613, t99, t658, t100, t2256, t107, tau0);
        let t2358 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk544::<F>(t661);
        let (t2359, t2362) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk545::<F>(t2357, t2358, t2256);
        let (t2363, t2366) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk546::<F>(t108, t2362, t101, t105, t2344, t2351, t2354, t2359, t656, t659, t97);
        let (t2367, t2371) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk547::<F>(t114, t2366, t655, t2335, t2336, t2341, t69);
    (t2341, t2344, t2349, t2350, t2357, t2358, t2359, t2362, t2363, t2366, t2367, t2371)
}
