//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 139/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk139<F: Float>(t211: F, t90: F, t238: F, t233: F, t345: F, t347: F, t351: F, t353: F, t241: F, t367: F, t46: F, t372: F, t374: F) -> (F, F, F, F, F) {
    let t607 = t211 * t90;
    let t622 = t238 * t238;
    let t623 = F::cast_from(1.0_f64) / t622;
    let t624 = t233 * t623;
    let t629 = -F::cast_from(0.1176575e1_f64) * t345 - F::cast_from(0.516475e0_f64) * t347 - F::cast_from(0.2103875e0_f64) * t351 - F::cast_from(0.104195e0_f64) * t353;
    let t630 = F::cast_from(1.0_f64) / t241;
    let t631 = t629 * t630;
    let t637 = t46 * t367;
    let t638 = t372 * t374;
    (t607, t624, t631, t637, t638)
}
