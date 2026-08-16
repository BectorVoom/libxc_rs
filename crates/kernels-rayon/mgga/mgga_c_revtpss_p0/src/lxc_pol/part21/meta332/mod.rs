//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta332 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1640;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta332(t1046: f64, t11262: f64, t1041: f64, t1038: f64, t3229: f64, t1036: f64, t1033: f64, t3169: f64, t3173: f64, t3140: f64, t989: f64, t3149: f64, t3160: f64, t2862: f64, t3128: f64, t1042: f64, t2853: f64, t3181: f64, t999: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11263, t11264, t11267, t11268) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1640(t1046, t11262, t1041, t1038, t3229, t1036, t1033);
        let (t11271, t11274, t11277, t11280, t11281, t11285) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1641(t3169, t3173, t3140, t989, t3149, t3160, t2862, t3128, t1042, t2853, t3181, t999);
    (t11263, t11264, t11267, t11268, t11271, t11274, t11277, t11280, t11281, t11285)
}
