//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta510 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1829;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta510<F: Float>(t10744: F, t2664: F, t7028: F, t25240: F, t2693: F, t2710: F, t228: F, t25273: F, t802: F, t25277: F, t2707: F, t25282: F, t9802: F, t243: F, t7021: F, t2732: F, t64: F, t9731: F, t826: F, t10631: F, t10886: F, t159: F, t8779: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t92963, t92966, t92968, t92969, t92971, t92975) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1829::<F>(t10744, t2664, t7028, t25240, t2693, t2710, t228, t25273, t802, t25277, t2707, t25282, t9802);
        let (t92979, t92986, t92988, t92991, t92993) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1830::<F>(t243, t7021, t2732, t64, t9731, t2710, t826, t10631, t10886, t7028, t159, t8779);
    (t92963, t92966, t92968, t92969, t92971, t92975, t92979, t92986, t92988, t92991, t92993)
}
