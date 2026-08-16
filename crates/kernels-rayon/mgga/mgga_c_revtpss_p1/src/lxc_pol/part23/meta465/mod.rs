//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1906;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1907;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta465(t19380: f64, t373: f64, t371: f64, t372: f64, t19463: f64, t366: f64, t3094: f64, t4186: f64, t4781: f64, t3092: f64, t4786: f64, t6092: f64, t11703: f64, t11710: f64, t6267: f64, t3091: f64, t4583: f64, t4823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19768, t19770, t19773) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1906(t19380, t373, t371, t372, t19463, t366);
        let (t19776, t19777, t19778, t19781, t19782, t19785, t19786, t19791) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1907(t3094, t4186, t4781, t3092, t4786, t6092, t11703, t11710, t6267, t3091, t4583, t4823);
    (t19768, t19770, t19773, t19776, t19777, t19778, t19781, t19782, t19785, t19786, t19791)
}
