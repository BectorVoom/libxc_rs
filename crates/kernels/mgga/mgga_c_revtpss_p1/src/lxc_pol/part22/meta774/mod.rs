//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta774 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2861;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta774<F: Float>(t3718: F, t3722: F, t44546: F, t3566: F, t3766: F, t5330: F, t12831: F, t12865: F, t1209: F, t13141: F, t17708: F, t11249: F, t3601: F, t13045: F, t3588: F, t371: F, t481: F, t482: F, t9291: F, t12627: F, t1284: F, t3624: F, t12640: F, t3555: F, t3781: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t44548, t44550, t44551, t44561, t44578, t44585) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2861::<F>(t3718, t3722, t44546, t3566, t3766, t5330, t12831, t12865, t1209, t13141, t17708, t11249, t3601);
        let (t44586, t44607, t44609, t44624, t44664) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2862::<F>(t13045, t3588, t371, t481, t482, t9291, t12627, t1284, t3624, t12640, t3555, t3781, t5330);
    (t44548, t44550, t44551, t44561, t44578, t44585, t44586, t44607, t44609, t44624, t44664)
}
