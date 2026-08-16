//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta824 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2941;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta824(t10026: f64, t48084: f64, t136: f64, t2457: f64, t3964: f64, t5710: f64, t221: f64, t9817: f64, t13792: f64, t13845: f64, t1882: f64, t9994: f64, t13793: f64, t13999: f64, t1868: f64, t3923: f64, t13872: f64, t3978: f64, t9921: f64, t1320: f64, t13632: f64, t13672: f64, t3860: f64, t5567: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48085, t48089, t48100, t48102, t48105) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2941(t10026, t48084, t136, t2457, t3964, t5710, t221, t9817, t13792, t13845, t1882, t9994);
        let (t48111, t48113, t48143, t48152, t48154, t48158) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2942(t13793, t13999, t1868, t3923, t13872, t221, t3978, t9921, t1320, t13632, t13672, t3860, t5567);
    (t48085, t48089, t48100, t48102, t48105, t48111, t48113, t48143, t48152, t48154, t48158)
}
