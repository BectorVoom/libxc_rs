//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta410 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1787;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1788;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta410<F: Float>(t14494: F, t6035: F, t14791: F, t2703: F, t5985: F, t10905: F, t5989: F, t10678: F, t10687: F, t10692: F, t14736: F, t14744: F, t14759: F, t14761: F, t14765: F, t14777: F, t2745: F, t5962: F, t854: F, t236: F, t807: F, t2476: F, t5966: F, t10717: F, t10719: F, t10723: F, t10746: F, t10749: F, t14780: F, t14783: F, t14817: F, t14820: F, t14823: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18333, t18334, t18338, t18340, t18343) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1787::<F>(t14494, t6035, t14791, t2703, t5985, t10905, t5989, t10678, t10687, t10692, t14736, t14744, t14759, t14761, t14765, t14777, t2745);
        let (t18348, t18349, t18350, t18352, t18353, t18354, t18361) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1788::<F>(t5962, t854, t236, t807, t2476, t5966, t10717, t10719, t10723, t10746, t10749, t14780, t14783, t14817, t14820, t14823);
    (t18333, t18334, t18338, t18340, t18343, t18348, t18349, t18350, t18352, t18353, t18354, t18361)
}
