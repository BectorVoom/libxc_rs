//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta666 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2464;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2465;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta666(t3075: f64, t3154: f64, t11671: f64, t11865: f64, t11697: f64, t11710: f64, t3091: f64, t11725: f64, t828: f64, t11706: f64, t11779: f64, t3215: f64, t225: f64, t42059: f64, t11675: f64, t11711: f64, t11666: f64, t4899: f64, t11262: f64, t3127: f64, t3129: f64, t11630: f64, t11633: f64, t3172: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43116, t43121, t43129, t43131, t43133, t43146) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2464(t3075, t3154, t11671, t11865, t11697, t11710, t3091, t11725, t828, t11706, t11779, t3215);
        let (t43154, t43169, t43172, t43204, t43211) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2465(t225, t42059, t11675, t11711, t11666, t11710, t4899, t11262, t3127, t3129, t11630, t11633, t3172);
    (t43116, t43121, t43129, t43131, t43133, t43146, t43154, t43169, t43172, t43204, t43211)
}
