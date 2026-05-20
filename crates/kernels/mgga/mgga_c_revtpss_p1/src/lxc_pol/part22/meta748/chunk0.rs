//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2821/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2821<F: Float>(t11384: F, t910: F, t275: F, t2872: F, t2922: F, t41245: F, t41306: F, t315: F, t41235: F, t11449: F, t941: F, t2941: F, t2966: F, t302: F) -> (F, F, F, F, F, F, F) {
    let t41583 = t910 * t11384;
    let t41588 = t275 / t2922 / t2872;
    let t41592 = F::cast_from(0.13388493827160493828e1_f64) * t41245;
    let t41610 = F::cast_from(0.31003950617283950618e1_f64) * t41306;
    let t41658 = t315 * t41235;
    let t41662 = t941 * t11449;
    let t41667 = t302 / t2966 / t2941;
    (t41583, t41588, t41592, t41610, t41658, t41662, t41667)
}
