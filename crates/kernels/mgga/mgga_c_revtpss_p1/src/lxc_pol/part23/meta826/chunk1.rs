//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2682/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2682<F: Float>(t1011: F, t15993: F, t18937: F, t127: F, t15700: F, t19979: F, t19981: F, t11859: F, t11922: F, t19635: F, t11875: F, t19640: F) -> (F, F, F, F) {
    let t66822 = t1011 * t15993 * t18937;
    let t66860 = t15700 * t127 * t19979 * t19981;
    let t66943 = t11859 * t11922 * t19635;
    let t66951 = t11875 * t11922 * t19640;
    (t66822, t66860, t66943, t66951)
}
