//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3193/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3193<F: Float>(t11262: F, t3711: F, t5278: F, t12640: F, t1811: F, t17807: F, t473: F, t3766: F, t5216: F, t13141: F, t1770: F, t1284: F, t17331: F) -> (F, F, F, F, F, F) {
    let t59426 = t3711 * t11262 * t5278;
    let t59464 = t12640 * t1811;
    let t59488 = t473 * t17807;
    let t59492 = t5216 * t3766;
    let t59498 = t1770 * t13141;
    let t59537 = t17331 * t1284;
    (t59426, t59464, t59488, t59492, t59498, t59537)
}
