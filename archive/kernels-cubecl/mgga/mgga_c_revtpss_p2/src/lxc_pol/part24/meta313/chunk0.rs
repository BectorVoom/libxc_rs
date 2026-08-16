//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1100/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1100<F: Float>(t22259: F, t4018: F, t14045: F, t6869: F, t3992: F, t2661: F, t221: F, t4019: F, t6874: F, t6864: F, t9918: F, t3930: F, t6876: F) -> (F, F, F, F, F, F, F) {
    let t22260 = t4018 * t22259;
    let t22262 = t14045 * t6869;
    let t22263 = t3992 * t22262;
    let t22264 = t2661 * t22263;
    let t22267 = t4019 * t221 * t6874;
    let t22268 = t4018 * t22267;
    let t22285 = t9918 * t6864;
    let t22292 = t3930 * t6876;
    (t22260, t22263, t22264, t22267, t22268, t22285, t22292)
}
