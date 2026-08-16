//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta392 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1445;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1446;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta392(t41372: f64, t916: f64, t270: f64, t276: f64, t39484: f64, t41383: f64, t2880: f64, t41386: f64, t11318: f64, t698: f64, t141: f64, t41314: f64, t930: f64, t11354: f64, t2881: f64, t2889: f64, t11315: f64, t11372: f64, t11358: f64, t11375: f64, t41316: f64, t41323: f64, t41353: f64, t41356: f64, t41359: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41396, t41402, t41404, t41406, t41409) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1445(t41372, t916, t270, t276, t39484, t41383, t2880, t41386, t11318, t698, t141, t41314, t930);
        let (t41412, t41414, t41417, t41419, t41421) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1446(t11354, t2881, t2889, t11315, t11372, t11358, t11375, t41316, t41323, t41353, t41356, t41359, t41396, t41402, t41404, t41406, t41409);
    (t41396, t41402, t41404, t41406, t41409, t41412, t41414, t41417, t41419, t41421)
}
