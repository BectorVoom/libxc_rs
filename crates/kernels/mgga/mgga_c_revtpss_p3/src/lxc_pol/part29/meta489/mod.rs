//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1774;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1775;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1776;
use chunk3::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1777;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta489<F: Float>(t1579: F, t7398: F, t7071: F, t72: F, t8006: F, t686: F, t25375: F, t25387: F, t27240: F, t25246: F, t25257: F, t25267: F, t26450: F, t26454: F, t27222: F, t27224: F, t27226: F, t27228: F, t27230: F, t27232: F, t27234: F, t27236: F, t27246: F, t27251: F, t27254: F, t27256: F, t25224: F, t25230: F, t25236: F, t25279: F, t26457: F, t26462: F, t26468: F, t26471: F, t27244: F, t27249: F, t27262: F, t225: F, t7997: F, t886: F, t27216: F, t7407: F, t213: F, t25383: F, t257: F, t26437: F, t26439: F, t26448: F, t26483: F, t26486: F, t4534: F, t7070: F, t7403: F, t7424: F, t7766: F, t8007: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28309, t28310, t28313, t28314, t28315, t28317, t28331) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1774::<F>(t1579, t7398, t7071, t72, t8006, t686, t25375, t25387, t27240, t25246, t25257, t25267, t26450, t26454, t27222, t27224, t27226, t27228, t27230, t27232, t27234, t27236);
        let t28339 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1775::<F>(t27246, t27251, t27254, t27256, t25224, t25230, t25236, t25279, t26457, t26462, t26468, t26471, t27244, t27249, t27262);
        let t28340 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1776::<F>(t28331, t28339);
        let (t28341, t28347, t28348, t28358) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1777::<F>(t225, t28340, t7997, t886, t7071, t27216, t7407, t213, t25383, t257, t26437, t26439, t26448, t26483, t26486, t28310, t28315, t28317, t4534, t7070, t7403, t7424, t7766, t8007);
    (t28309, t28310, t28313, t28314, t28340, t28341, t28347, t28348, t28358)
}
