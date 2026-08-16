//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta95 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk647;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk648;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk649;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk650;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk651;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk652;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk653;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta95(t676: f64, t886: f64, t123: f64, t2465: f64, t215: f64, t685: f64, t788: f64, t787: f64, t206: f64, t242: f64, t240: f64, t72: f64, t225: f64, t786: f64, t27: f64, t823: f64, t136: f64, t826: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t2466, t2467, t2468, t2470) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk647(t676, t886, t123, t2465, t215, t685);
        let t2471 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk648(t2470, t788);
        let (t2473, t2475) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk649(t2471, t787, t206, t242);
        let (t2476, t2477) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk650(t240, t2475, t72);
        let t2482 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk651(t225, t786);
        let t2484 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk652(t2482, t27, t823);
        let t2485 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk653(t136, t826);
    (t2466, t2467, t2468, t2470, t2471, t2473, t2475, t2476, t2477, t2482, t2484, t2485)
}
