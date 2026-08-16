//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta826 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2681;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2682;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta826<F: Float>(t3091: F, t43240: F, t6267: F, t16088: F, t380: F, t4746: F, t1065: F, t372: F, t6299: F, t3105: F, t6317: F, t15794: F, t15926: F, t1011: F, t15993: F, t18937: F, t127: F, t15700: F, t19979: F, t19981: F, t11859: F, t11922: F, t19635: F, t11875: F, t19640: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t66763, t66766, t66777, t66784, t66814) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2681::<F>(t3091, t43240, t6267, t16088, t380, t4746, t1065, t372, t6299, t3105, t6317, t15794, t15926);
        let (t66822, t66860, t66943, t66951) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2682::<F>(t1011, t15993, t18937, t127, t15700, t19979, t19981, t11859, t11922, t19635, t11875, t19640);
    (t66763, t66766, t66777, t66784, t66814, t66822, t66860, t66943, t66951)
}
