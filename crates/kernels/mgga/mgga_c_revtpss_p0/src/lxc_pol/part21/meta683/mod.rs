//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2497;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta683<F: Float>(t3718: F, t3722: F, t44546: F, t3566: F, t3766: F, t5330: F, t12646: F, t12915: F, t247: F, t5384: F, t12831: F, t12865: F, t1260: F, t12889: F, t12886: F, t3647: F, t1209: F, t13141: F, t17708: F, t12832: F, t12917: F, t11249: F, t3601: F, t13045: F, t3588: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t44548, t44551, t44559, t44561) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2497::<F>(t3718, t3722, t44546, t3566, t3766, t5330, t12646, t12915, t247, t5384, t12831, t12865);
        let (t44568, t44571, t44578, t44583, t44585, t44586) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2498::<F>(t1260, t12889, t12886, t3647, t1209, t13141, t17708, t12832, t12917, t11249, t3601, t13045, t3588);
    (t44548, t44551, t44559, t44561, t44568, t44571, t44578, t44583, t44585, t44586)
}
