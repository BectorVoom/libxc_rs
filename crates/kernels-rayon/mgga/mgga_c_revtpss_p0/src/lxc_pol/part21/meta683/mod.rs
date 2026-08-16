//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta683 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2497;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2498;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta683(t3718: f64, t3722: f64, t44546: f64, t3566: f64, t3766: f64, t5330: f64, t12646: f64, t12915: f64, t247: f64, t5384: f64, t12831: f64, t12865: f64, t1260: f64, t12889: f64, t12886: f64, t3647: f64, t1209: f64, t13141: f64, t17708: f64, t12832: f64, t12917: f64, t11249: f64, t3601: f64, t13045: f64, t3588: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t44548, t44551, t44559, t44561) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2497(t3718, t3722, t44546, t3566, t3766, t5330, t12646, t12915, t247, t5384, t12831, t12865);
        let (t44568, t44571, t44578, t44583, t44585, t44586) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2498(t1260, t12889, t12886, t3647, t1209, t13141, t17708, t12832, t12917, t11249, t3601, t13045, t3588);
    (t44548, t44551, t44559, t44561, t44568, t44571, t44578, t44583, t44585, t44586)
}
