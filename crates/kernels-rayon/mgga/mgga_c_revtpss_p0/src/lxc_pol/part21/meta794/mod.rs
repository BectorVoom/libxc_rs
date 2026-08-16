//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta794 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2873;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta794(t15389: f64, t2918: f64, t2924: f64, t11387: f64, t4631: f64, t11385: f64, t2875: f64, t51840: f64, t51844: f64, t51846: f64, t52141: f64, t52146: f64, t52150: f64, t52153: f64, t52156: f64, t52159: f64, t11379: f64, t4635: f64, t11300: f64, t1609: f64, t41499: f64, t41502: f64, t11528: f64, t15383: f64, t15386: f64, t41883: f64, t11294: f64, t15393: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t52162, t52166, t52167) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2873(t15389, t2918, t2924, t11387, t4631, t11385, t2875, t51840, t51844, t51846, t52141, t52146, t52150, t52153, t52156, t52159);
        let (t52170, t52174, t52176, t52178, t52180) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2874(t11379, t2924, t4635, t11300, t1609, t41499, t41502, t11528, t15383, t15386, t41883, t11294, t15393);
    (t52162, t52166, t52167, t52170, t52174, t52176, t52178, t52180)
}
