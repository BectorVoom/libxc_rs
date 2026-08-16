//! MGGA_C_REVTPSS lxc pol kernel — _part42_v4rho3tau_5 meta199 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk804;
use chunk1::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk805;
use chunk2::mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk806;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_meta199<F: Float>(t1214: F, t471: F, t5351: F, t3720: F, t140: F, t1781: F, t1222: F, t127: F, t1789: F, t371: F, t1235: F, t1219: F, t1778: F, t1225: F, t4186: F, t1012: F, t3657: F, t3658: F, t3679: F, t3684: F, t3718: F, t5340: F, t5343: F, t5348: F, t1010: F, t1480: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5352, t5353, t5354, t5357, t5358, t5362, t5363, t5366) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk804::<F>(t1214, t471, t5351, t3720, t140, t1781, t1222, t127, t1789, t371, t1235, t1219, t1778);
        let (t5368, t5369, t5372) = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk805::<F>(t1225, t4186, t1012, t1222, t3657, t3658, t3679, t3684, t3718, t5340, t5343, t5348, t5354, t5358, t5363, t5366);
        let t5373 = mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk806::<F>(t1010, t1480);
    (t5352, t5353, t5354, t5357, t5358, t5362, t5363, t5366, t5368, t5369, t5372, t5373)
}
