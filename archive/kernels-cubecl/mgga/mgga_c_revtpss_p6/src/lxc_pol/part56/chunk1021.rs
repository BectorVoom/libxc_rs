//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1021/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1021<F: Float>(t90: F, t29: F, t560: F, t9655: F, t4146: F, t550: F, t9794: F, t5778: F, t9593: F, t243: F, t2246: F, t4171: F) -> (F, F, F, F, F, F, F) {
    let t45970 = t90 * t90;
    let t45972 = t29 / t45970;
    let t46361 = F::cast_from(1.0_f64) / t9655 / t560;
    let t47671 = t4146 * t4146;
    let t47672 = F::cast_from(1.0_f64) / t47671;
    let t49068 = t9794 * t550;
    let t49575 = t5778 * t9593;
    let t51076 = t9794 * t243;
    let t60221 = t4171 * t2246;
    (t45972, t46361, t47672, t49068, t49575, t51076, t60221)
}
