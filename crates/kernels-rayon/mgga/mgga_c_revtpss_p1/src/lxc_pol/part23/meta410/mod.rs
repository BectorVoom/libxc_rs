//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1787;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta410(t14494: f64, t6035: f64, t14791: f64, t2703: f64, t5985: f64, t10905: f64, t5989: f64, t10678: f64, t10687: f64, t10692: f64, t14736: f64, t14744: f64, t14759: f64, t14761: f64, t14765: f64, t14777: f64, t2745: f64, t5962: f64, t854: f64, t236: f64, t807: f64, t2476: f64, t5966: f64, t10717: f64, t10719: f64, t10723: f64, t10746: f64, t10749: f64, t14780: f64, t14783: f64, t14817: f64, t14820: f64, t14823: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t18333, t18334, t18338, t18340, t18343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1787(t14494, t6035, t14791, t2703, t5985, t10905, t5989, t10678, t10687, t10692, t14736, t14744, t14759, t14761, t14765, t14777, t2745);
        let (t18348, t18349, t18350, t18352, t18353, t18354, t18361) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1788(t5962, t854, t236, t807, t2476, t5966, t10717, t10719, t10723, t10746, t10749, t14780, t14783, t14817, t14820, t14823);
    (t18333, t18334, t18338, t18340, t18343, t18348, t18349, t18350, t18352, t18353, t18354, t18361)
}
