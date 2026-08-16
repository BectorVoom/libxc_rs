//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta826 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2681;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta826(t3091: f64, t43240: f64, t6267: f64, t16088: f64, t380: f64, t4746: f64, t1065: f64, t372: f64, t6299: f64, t3105: f64, t6317: f64, t15794: f64, t15926: f64, t1011: f64, t15993: f64, t18937: f64, t127: f64, t15700: f64, t19979: f64, t19981: f64, t11859: f64, t11922: f64, t19635: f64, t11875: f64, t19640: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t66763, t66766, t66777, t66784, t66814) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2681(t3091, t43240, t6267, t16088, t380, t4746, t1065, t372, t6299, t3105, t6317, t15794, t15926);
        let (t66822, t66860, t66943, t66951) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2682(t1011, t15993, t18937, t127, t15700, t19979, t19981, t11859, t11922, t19635, t11875, t19640);
    (t66763, t66766, t66777, t66784, t66814, t66822, t66860, t66943, t66951)
}
