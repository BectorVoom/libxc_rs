//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta692 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2695;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2696;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta692(t22046: f64, t3936: f64, t3938: f64, t5659: f64, t5673: f64, t5674: f64, t1399: f64, t125: f64, t6836: f64, t9955: f64, t1413: f64, t6816: f64, t547: f64, t807: f64, t4011: f64, t1353: f64, t6883: f64, t800: f64, t13832: f64, t13851: f64, t13858: f64, t3934: f64, t3944: f64, t9739: f64, t9742: f64, t9766: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22107, t22111, t22115, t22118, t22120, t22125) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2695(t22046, t3936, t3938, t5659, t5673, t5674, t1399, t125, t6836, t9955, t1413, t6816);
        let (t22126, t22129, t22130, t22135, t22140) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2696(t22125, t547, t807, t4011, t6836, t1353, t6883, t800, t13832, t13851, t13858, t22107, t22111, t22115, t22120, t3934, t3944, t9739, t9742, t9766);
    (t22107, t22111, t22115, t22118, t22120, t22125, t22126, t22129, t22130, t22135, t22140)
}
