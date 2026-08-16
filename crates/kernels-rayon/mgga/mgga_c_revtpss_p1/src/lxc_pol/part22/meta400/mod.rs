//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta400 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1991;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta400(t13847: f64, t13848: f64, t1399: f64, t9816: f64, t2713: f64, t3964: f64, t5617: f64, t1872: f64, t3829: f64, t800: f64, t124: f64, t13716: f64, t5686: f64, t9744: f64, t1353: f64, t5689: f64, t3889: f64, t1370: f64, t3944: f64, t9748: f64, t9924: f64, t9926: f64, t9932: f64, t9937: f64, t9953: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14005, t14007, t14013, t14016, t14019) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1991(t13847, t13848, t1399, t9816, t2713, t3964, t5617, t1872, t3829, t800, t124, t13716);
        let (t14020, t14024, t14026, t14030, t14033) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1992(t14019, t800, t5686, t9744, t1353, t5689, t1872, t3889, t1370, t14007, t14013, t14016, t3944, t9748, t9924, t9926, t9932, t9937, t9953);
    (t14005, t14007, t14013, t14016, t14020, t14024, t14026, t14030, t14033)
}
