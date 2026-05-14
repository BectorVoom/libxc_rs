//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 133/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk133<F: Float>(t213: F, t218: F, t211: F, t90: F, t64: F, t215: F, t220: F, t43: F, t238: F, t233: F, t345: F, t347: F, t351: F, t353: F, t241: F, t367: F, t46: F, t372: F, t374: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t607 = t211 * t90;
    let t608 = t64 - t607;
    let t611 = piecewise3(t214, 0.0, 4.0 / 3.0 * t215 * t608);
    let t612 = -t608;
    let t615 = piecewise3(t219, 0.0, 4.0 / 3.0 * t220 * t612);
    let t617 = (t611 + t615) * t43;
    let t622 = t238 * t238;
    let t623 = 1.0 / t622;
    let t624 = t233 * t623;
    let t629 = -0.1176575e1 * t345 - 0.516475e0 * t347 - 0.2103875e0 * t351 - 0.104195e0 * t353;
    let t630 = 1.0 / t241;
    let t631 = t629 * t630;
    let t637 = t46 * t367;
    let t638 = t372 * t374;
    (t617, t624, t631, t637, t638)
}
