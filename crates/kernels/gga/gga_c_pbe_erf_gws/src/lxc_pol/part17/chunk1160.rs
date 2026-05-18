//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1160/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1160<F: Float>(t14754: F, t3972: F, t4182: F, t810: F, t2376: F, t2409: F, t1112: F, t331: F, t2306: F, t3074: F, t833: F, t4157: F, t4414: F) -> (F, F, F, F, F, F, F, F) {
    let t14755 = t3972 * t14754;
    let t14757 = t4182 * t810;
    let t14759 = t2409 * t2376 * t14757;
    let t14765 = t1112 * t331;
    let t14766 = t2306 * t14765;
    let t14767 = t3074 * t14766;
    let t14768 = t14767 * t833;
    let t14770 = t4414 * t4157;
    (t14755, t14757, t14759, t14765, t14766, t14767, t14768, t14770)
}
