//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta446 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1668;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1669;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1670;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1671;
use chunk4::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1672;
use chunk5::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1673;
use chunk6::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1674;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta446(t25372: f64, t25374: f64, t1955: f64, t25308: f64, t251: f64, t7063: f64, t2769: f64, t7056: f64, t231: f64, t836: f64, t886: f64, t233: f64, t867: f64, t2760: f64, t1957: f64, t822: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t25375 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1668(t25372, t25374);
        let t25383 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1669(t1955, t25308);
        let t25386 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1670(t251, t7063);
        let t25387 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1671(t25374, t25386);
        let (t25390, t25391) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1672(t2769, t7056, t1955);
        let (t25394, t25402, t25407, t25410) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1673(t231, t836, t886, t233, t867, t1955, t2760, t1957, t822);
        let t25411 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1674(t25386, t25410);
    (t25375, t25383, t25386, t25387, t25390, t25391, t25394, t25402, t25407, t25410, t25411)
}
