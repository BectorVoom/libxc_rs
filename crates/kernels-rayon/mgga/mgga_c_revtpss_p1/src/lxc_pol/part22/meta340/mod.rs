//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta340 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1809;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1810;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1811;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta340(t11249: f64, t3154: f64, t246: f64, t676: f64, t1046: f64, t1041: f64, t1038: f64, t3229: f64, t1036: f64, t1033: f64, t3169: f64, t3173: f64, t3140: f64, t989: f64, t3149: f64, t3160: f64, t2866: f64, t914: f64, t2923: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11250, t11262) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1809(t11249, t3154, t246, t676);
        let (t11263, t11264, t11267, t11268, t11271, t11273, t11274) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1810(t1046, t11262, t1041, t1038, t3229, t1036, t1033, t3169, t3173, t3140, t989, t3149);
        let (t11277, t11289, t11294) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1811(t11273, t3160, t2866, t914, t2923, t910);
    (t11250, t11262, t11263, t11264, t11267, t11268, t11271, t11273, t11274, t11277, t11289, t11294)
}
