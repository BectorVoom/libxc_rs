//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta951 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3193;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3194;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta951<F: Float>(t11262: F, t3711: F, t5278: F, t12640: F, t1811: F, t17807: F, t473: F, t3766: F, t5216: F, t13141: F, t1770: F, t1284: F, t17331: F, t13126: F, t1269: F, t460: F, t12050: F, t17710: F, t17191: F, t3555: F, t13147: F, t1209: F, t21455: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t59426, t59464, t59488, t59492, t59498, t59537) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3193::<F>(t11262, t3711, t5278, t12640, t1811, t17807, t473, t3766, t5216, t13141, t1770, t1284, t17331);
        let (t59550, t59591, t59650, t59657, t59671, t59674) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3194::<F>(t13126, t1770, t1269, t13141, t460, t12050, t17710, t17191, t3555, t13147, t1209, t21455);
    (t59426, t59464, t59488, t59492, t59498, t59537, t59550, t59591, t59650, t59657, t59671, t59674)
}
