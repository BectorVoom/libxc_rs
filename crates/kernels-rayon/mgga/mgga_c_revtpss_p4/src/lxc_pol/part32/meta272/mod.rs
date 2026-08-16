//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta272 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1149;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1150;
use chunk2::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1151;
use chunk3::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1152;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta272(t3: f64, t8113: f64, param_d: f64, t1518: f64, t7553: f64, t117: f64, t7983: f64, t1916: f64, t1918: f64, t2113: f64, t2115: f64, t572: f64, t573: f64, t587: f64, t65: f64, t197: f64, t532: f64, t1450: f64, t2106: f64, t143: f64, t2580: f64, t130: f64, t2566: f64, t700: f64, t2584: f64, t121: f64, t131: f64, t141: f64, t22: f64, t2456: f64, t624: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t8114, t8118) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1149(t3, t8113, param_d);
        let (t8124, t8127, t8130, t8779) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1150(t1518, t7553, t117, t7983, t1916, t1918, t2113, t2115, t572, t573, t8118, t587, t65);
        let (t8995, t9069, t9275, t9278) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1151(t197, t532, t1450, t2106, t143, t2580, t130, t2566, t700, t2584);
        let (t9283, t9285) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1152(t121, t131, t141, t22, t2456, t624);
    (t8114, t8118, t8124, t8127, t8130, t8779, t8995, t9069, t9275, t9278, t9283, t9285)
}
