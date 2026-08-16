//! MGGA_C_REVTPSS lxc pol kernel — _part41_v4rho3tau_4 meta326 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1113;
use chunk1::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1114;
use chunk2::mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_meta326(t1469: f64, t2609: f64, t706: f64, t1568: f64, t785: f64, t780: f64, t2439: f64, t212: f64, t4469: f64, t689: f64, t1579: f64, t2769: f64, t886: f64, t252: f64, t2782: f64, t2470: f64, t4480: f64, t2465: f64, t1558: f64, t836: f64, t231: f64, t2797: f64, t860: f64, t2783: f64, t251: f64, t4423: f64, t10073: f64, t4496: f64, t10542: f64, t4500: f64, t4424: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14441, t14474, t14479, t14480) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1113(t1469, t2609, t706, t1568, t785, t780, t2439, t212, t4469, t689, t1579, t2769);
        let (t14484, t14486, t14494, t14498, t14502) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1114(t14480, t886, t252, t2782, t2470, t4480, t2465, t1558, t836, t231, t2797, t860);
        let (t14506, t14511, t14512, t14518, t14519) = mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1115(t14502, t231, t2783, t2782, t251, t4423, t10073, t4496, t10542, t4500, t4424, t72);
    (t14441, t14474, t14479, t14484, t14486, t14494, t14498, t14506, t14511, t14512, t14518, t14519)
}
