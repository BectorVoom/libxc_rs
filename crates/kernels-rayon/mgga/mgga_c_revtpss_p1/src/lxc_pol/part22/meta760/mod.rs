//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta760 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2840;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta760(t11670: f64, t11772: f64, t3114: f64, t11773: f64, t11926: f64, t11858: f64, t15688: f64, t16102: f64, t3155: f64, t12077: f64, t15905: f64, t994: f64, t3075: f64, t3154: f64, t11671: f64, t11865: f64, t11725: f64, t828: f64, t11660: f64, t2258: f64, t3204: f64, t3230: f64, t225: f64, t42059: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43065, t43066, t43069, t43082, t43085, t43105) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2840(t11670, t11772, t3114, t11773, t11926, t11858, t15688, t16102, t3155, t12077, t15905, t994);
        let (t43116, t43121, t43131, t43139, t43151, t43154) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2841(t3075, t3154, t11671, t11865, t11725, t828, t11660, t2258, t3204, t3230, t225, t42059);
    (t43065, t43066, t43069, t43082, t43085, t43105, t43116, t43121, t43131, t43139, t43151, t43154)
}
