//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta101 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk606;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk607;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk608;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk609;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta101(t676: f64, t886: f64, t123: f64, t2465: f64, t215: f64, t685: f64, t788: f64, t787: f64, t206: f64, t242: f64, t240: f64, t72: f64, t225: f64, t786: f64, t27: f64, t823: f64, t136: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2466, t2467, t2468, t2470) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk606(t676, t886, t123, t2465, t215, t685);
        let t2471 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk607(t2470, t788);
        let (t2473, t2475, t2476, t2477, t2482) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk608(t2471, t787, t206, t242, t240, t72, t225, t786);
        let t2484 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk609(t2482, t27, t823);
        let t2485 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk610(t136, t826);
    (t2466, t2467, t2468, t2470, t2471, t2473, t2475, t2476, t2477, t2482, t2484, t2485)
}
