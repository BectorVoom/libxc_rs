//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk959;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk960;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk961;
use chunk3::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk962;
use chunk4::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk963;
use chunk5::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk964;
use chunk6::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk965;
use chunk7::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta258<F: Float>(t101: F, t96: F, t665: F, t43: F, t655: F, t69: F, t100: F, t114: F, t658: F, t8257: F, t8258: F, t508: F, t569: F, t1453: F, t2178: F, t1312: F, t2179: F, t2181: F, t2322: F, t4254: F, t5523: F, t651: F, t8254: F, t3: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t8259 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk959::<F>(t101, t96);
        let (t8260, t8264, t8267) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk960::<F>(t665, t8259, t101, t43, t655, t69);
        let t8268 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk961::<F>(t100, t96);
        let (t8269, t8273) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk962::<F>(t114, t658, t8268, t69, t8257, t8258, t8260, t8264, t8267);
        let t8274 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk963::<F>(t508, t8273);
        let t8278 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk964::<F>(t569, t8273);
        let t8280 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk965::<F>(t1453, t2178);
        let (t8283, t8284) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk966::<F>(t1312, t2179, t2181, t2322, t4254, t5523, t651, t8254, t8274, t8278, t8280, t3);
    (t8259, t8260, t8264, t8267, t8268, t8269, t8273, t8274, t8278, t8280, t8283, t8284)
}
