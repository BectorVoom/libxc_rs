//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta667 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2466;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2467;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta667(t11988: f64, t3106: f64, t271: f64, t2852: f64, t1054: f64, t11970: f64, t11986: f64, t828: f64, t3091: f64, t3096: f64, t12097: f64, t3090: f64, t11280: f64, t3127: f64, t3172: f64, t11870: f64, t11922: f64, t3115: f64, t11631: f64, t3133: f64, t1086: f64, t11223: f64, t11866: f64, t11923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43215, t43222, t43238, t43240, t43242, t43244) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2466(t11988, t3106, t271, t2852, t1054, t11970, t11986, t828, t3091, t3096, t12097, t3090);
        let (t43266, t43277, t43279, t43285, t43288) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2467(t11280, t3127, t3172, t11870, t11922, t3115, t11631, t3133, t1086, t11223, t3090, t11866, t11923);
    (t43215, t43222, t43238, t43240, t43242, t43244, t43266, t43277, t43279, t43285, t43288)
}
