//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta754 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2829;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2830;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta754(t11273: f64, t11998: f64, t1062: f64, t11782: f64, t11853: f64, t828: f64, t3229: f64, t360: f64, t3089: f64, t1087: f64, t1024: f64, t12003: f64, t3181: f64, t675: f64, t1063: f64, t247: f64, t2853: f64, t283: f64, t2852: f64, t1025: f64, t3218: f64, t371: f64, t676: f64, t11144: f64, t3252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t42371, t42391, t42410, t42416, t42417, t42425) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2829(t11273, t11998, t1062, t11782, t11853, t828, t3229, t360, t3089, t1087, t1024, t12003);
        let (t42447, t42450, t42471, t42481, t42518) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2830(t3181, t675, t1063, t247, t2853, t283, t2852, t1025, t3218, t371, t676, t11144, t3252);
    (t42371, t42391, t42410, t42416, t42417, t42425, t42447, t42450, t42471, t42481, t42518)
}
