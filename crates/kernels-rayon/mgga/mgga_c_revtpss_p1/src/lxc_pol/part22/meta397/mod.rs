//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta397 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1982;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1983;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1984;
use chunk3::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1985;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta397(t13920: f64, t543: f64, t1390: f64, t828: f64, t1398: f64, t1882: f64, t3938: f64, t13789: f64, t13869: f64, t13874: f64, t1388: f64, t13880: f64, t1410: f64, t3934: f64, t9753: f64, t9762: f64, t9766: f64, t9771: f64, t9776: f64, t9780: f64, t9786: f64, t9791: f64, t4057: f64, t5673: f64, t5674: f64, t13848: f64, t9818: f64, t9816: f64, t125: f64, t5658: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t13921 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1982(t13920, t543);
        let (t13923, t13926) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1983(t1390, t13921, t828, t1398, t1882);
        let (t13927, t13928, t13931) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1984(t13926, t3938, t13789, t13869, t13874, t1388, t13880, t13923, t1410, t3934, t9753, t9762, t9766, t9771, t9776, t9780, t9786, t9791);
        let (t13937, t13941, t13943, t13944) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1985(t4057, t5673, t5674, t13848, t3938, t9818, t9816, t125, t5658);
    (t13921, t13923, t13926, t13927, t13928, t13931, t13937, t13941, t13943, t13944)
}
