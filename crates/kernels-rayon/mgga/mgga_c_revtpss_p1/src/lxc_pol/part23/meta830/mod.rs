//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta830 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2689;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2690;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta830(t20020: f64, t3224: f64, t1025: f64, t127: f64, t19768: f64, t371: f64, t225: f64, t64686: f64, t366: f64, t64907: f64, t19773: f64, t3215: f64, t11922: f64, t16067: f64, t19721: f64, t19566: f64, t3090: f64, t1086: f64, t19462: f64, t19972: f64, t4892: f64, t19658: f64, t3124: f64, t19882: f64, t3106: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t67493, t67499, t67501, t67516, t67521) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2689(t20020, t3224, t1025, t127, t19768, t371, t225, t64686, t366, t64907, t19773, t3215);
        let (t67526, t67528, t67551, t67560, t67568, t67571) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2690(t11922, t16067, t19721, t19566, t3090, t1086, t19462, t19972, t4892, t19658, t3124, t19882, t3106);
    (t67493, t67499, t67501, t67516, t67521, t67526, t67528, t67551, t67560, t67568, t67571)
}
