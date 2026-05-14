//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 869/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk869<F: Float>(t6800: F, t72: F, t757: F, t1317: F, t6801: F, t1320: F, t749: F, t512: F, t177: F, t762: F, t221: F, t4019: F, t6844: F, t4018: F, t14045: F, t6869: F) -> (F, F, F, F, F, F, F, F) {
    let t22185 = t6800 * t72;
    let t22186 = t22185 * t757;
    let t22188 = t1317 * t6801;
    let t22191 = t1320 * t6801;
    let t22195 = t6800 * t749;
    let t22196 = t512 * t22195;
    let t22212 = t6800 * t177;
    let t22213 = t22212 * t762;
    let t22259 = t4019 * t221 * t6844;
    let t22260 = t4018 * t22259;
    let t22262 = t14045 * t6869;
    (t22186, t22188, t22191, t22196, t22213, t22259, t22260, t22262)
}
