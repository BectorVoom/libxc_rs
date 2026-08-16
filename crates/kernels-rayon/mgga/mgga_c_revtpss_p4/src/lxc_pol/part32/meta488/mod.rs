//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1737;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1738;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1739;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1740;
use chunk4::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta488(t1579: f64, t7398: f64, t7071: f64, t72: f64, t8006: f64, t686: f64, t25375: f64, t25387: f64, t27240: f64, t25246: f64, t25257: f64, t25267: f64, t26450: f64, t26454: f64, t27222: f64, t27224: f64, t27226: f64, t27228: f64, t27230: f64, t27232: f64, t27234: f64, t27236: f64, t27246: f64, t27251: f64, t27254: f64, t27256: f64, t25224: f64, t25230: f64, t25236: f64, t25279: f64, t26457: f64, t26462: f64, t26468: f64, t26471: f64, t27244: f64, t27249: f64, t27262: f64, t225: f64, t7997: f64, t886: f64, t27216: f64, t7407: f64, t213: f64, t25383: f64, t257: f64, t26437: f64, t26439: f64, t26448: f64, t26483: f64, t26486: f64, t4534: f64, t7070: f64, t7403: f64, t7424: f64, t7766: f64, t8007: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28309, t28310, t28313, t28314) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1737(t1579, t7398, t7071, t72, t8006, t686);
        let (t28315, t28317, t28331) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1738(t25375, t28314, t25387, t27240, t25246, t25257, t25267, t26450, t26454, t27222, t27224, t27226, t27228, t27230, t27232, t27234, t27236);
        let t28339 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1739(t27246, t27251, t27254, t27256, t25224, t25230, t25236, t25279, t26457, t26462, t26468, t26471, t27244, t27249, t27262);
        let t28340 = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1740(t28331, t28339);
        let (t28341, t28347, t28348, t28352, t28358) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1741(t225, t28340, t7997, t886, t7071, t27216, t7407, t213, t25383, t257, t26437, t26439, t26448, t26483, t26486, t28310, t28315, t28317, t4534, t7070, t7403, t7424, t7766, t8007);
    (t28309, t28310, t28313, t28314, t28315, t28317, t28340, t28341, t28347, t28348, t28352, t28358)
}
