//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta761 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2842;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2843;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta761(t2857: f64, t3154: f64, t2251: f64, t11262: f64, t3127: f64, t3129: f64, t11988: f64, t3106: f64, t271: f64, t2852: f64, t1054: f64, t11970: f64, t11986: f64, t828: f64, t3091: f64, t3096: f64, t12097: f64, t3090: f64, t11273: f64, t12012: f64, t11631: f64, t3133: f64, t1086: f64, t11223: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43174, t43175, t43204, t43215, t43222, t43238) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2842(t2857, t3154, t2251, t11262, t3127, t3129, t11988, t3106, t271, t2852, t1054, t11970);
        let (t43240, t43242, t43244, t43268, t43279, t43285) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2843(t11986, t828, t3091, t3096, t12097, t3090, t11273, t12012, t11631, t3133, t1086, t11223);
    (t43174, t43175, t43204, t43215, t43222, t43238, t43240, t43242, t43244, t43268, t43279, t43285)
}
