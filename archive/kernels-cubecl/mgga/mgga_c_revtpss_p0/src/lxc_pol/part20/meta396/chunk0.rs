//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1457/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1457<F: Float>(t11289: F, t2919: F, t2866: F, t2923: F, t2927: F, t11380: F, t2869: F, t11384: F, t910: F, t11388: F, t275: F, t2872: F, t2922: F) -> (F, F, F, F, F) {
    let t41577 = F::cast_from(6.0_f64) * t11289 * t2919;
    let t41578 = t2866 * t2923;
    let t41580 = F::cast_from(0.96491876992155210402e2_f64) * t41578 * t2927;
    let t41582 = F::cast_from(4.0_f64) * t2869 * t11380;
    let t41583 = t910 * t11384;
    let t41585 = F::cast_from(0.2069040516770936012e4_f64) * t41583 * t11388;
    let t41588 = t275 / t2922 / t2872;
    (t41577, t41580, t41582, t41585, t41588)
}
