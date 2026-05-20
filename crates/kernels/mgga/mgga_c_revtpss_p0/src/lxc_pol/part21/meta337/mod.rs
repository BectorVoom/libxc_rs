//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta337 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1650;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1651;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1652;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta337<F: Float>(t2915: F, t698: F, t11315: F, t916: F, t2880: F, t918: F, t2889: F, t2897: F, t11134: F, t11136: F, t11138: F, t11140: F, t11147: F, t11153: F, t11171: F, t11356: F, t11359: F, t11366: F, t11349: F, t935: F, t915: F, t2922: F, t913: F, t275: F) -> (F, F, F, F, F, F, F, F, F) {
        let t11368 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1650::<F>(t2915, t698);
        let (t11370, t11373, t11376, t11378) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1651::<F>(t11315, t916, t2880, t918, t2889, t2897, t11134, t11136, t11138, t11140, t11147, t11153, t11171, t11356, t11359, t11366, t11368);
        let (t11379, t11380, t11382, t11384, t11385) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1652::<F>(t11349, t11378, t935, t915, t2922, t913, t275);
    (t11368, t11370, t11373, t11376, t11379, t11380, t11382, t11384, t11385)
}
