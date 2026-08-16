//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3848/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3848<F: Float>(t21969: F, t221: F, t3978: F, t3979: F, t4010: F, t6816: F, t1353: F, t13767: F, t2661: F, t22027: F, t9775: F, t22252: F, t3992: F, t543: F, t550: F) -> (F, F, F, F, F) {
    let t74010 = t3978 * t3979 * t221 * t21969;
    let t74012 = t4010 * t6816;
    let t74015 = t2661 * t13767 * t74012 * t1353;
    let t74017 = t9775 * t22027;
    let t74022 = t2661 * t3992 * t550 * t22252 * t543;
    (t74010, t74012, t74015, t74017, t74022)
}
