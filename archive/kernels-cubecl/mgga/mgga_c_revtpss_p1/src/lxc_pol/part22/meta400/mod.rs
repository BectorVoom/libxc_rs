//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1991;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta400<F: Float>(t13847: F, t13848: F, t1399: F, t9816: F, t2713: F, t3964: F, t5617: F, t1872: F, t3829: F, t800: F, t124: F, t13716: F, t5686: F, t9744: F, t1353: F, t5689: F, t3889: F, t1370: F, t3944: F, t9748: F, t9924: F, t9926: F, t9932: F, t9937: F, t9953: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14005, t14007, t14013, t14016, t14019) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1991::<F>(t13847, t13848, t1399, t9816, t2713, t3964, t5617, t1872, t3829, t800, t124, t13716);
        let (t14020, t14024, t14026, t14030, t14033) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1992::<F>(t14019, t800, t5686, t9744, t1353, t5689, t1872, t3889, t1370, t14007, t14013, t14016, t3944, t9748, t9924, t9926, t9932, t9937, t9953);
    (t14005, t14007, t14013, t14016, t14020, t14024, t14026, t14030, t14033)
}
