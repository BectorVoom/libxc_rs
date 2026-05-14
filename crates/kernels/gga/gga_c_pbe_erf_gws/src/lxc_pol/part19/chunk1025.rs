//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1025/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1025<F: Float>(t14138: F, t14733: F, t1173: F, t3202: F, t14001: F, t4130: F, t13953: F, t4135: F, t3294: F, t3975: F, t3972: F, t1112: F, t331: F, t2306: F, t3074: F, t833: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14734 = t14733 * t14138;
    let t14737 = t1173 * t3202;
    let t14745 = t14001 * t4130;
    let t14752 = t13953 * t4135;
    let t14754 = t3975 * t3294;
    let t14755 = t3972 * t14754;
    let t14765 = t1112 * t331;
    let t14766 = t2306 * t14765;
    let t14767 = t3074 * t14766;
    let t14768 = t14767 * t833;
    (t14734, t14737, t14745, t14752, t14754, t14755, t14765, t14767, t14768)
}
