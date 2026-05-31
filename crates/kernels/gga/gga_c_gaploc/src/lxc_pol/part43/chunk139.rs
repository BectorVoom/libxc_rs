//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 139/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk139<F: Float>(t213: F, t218: F, t211: F, t90: F, t64: F, t215: F, t220: F, t43: F, t238: F, t233: F, t345: F, t347: F, t351: F, t353: F, zeta_threshold: F) -> (F, F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t607 = t211 * t90;
    let t608 = t64 - t607;
    let t611 = piecewise3::<F>(t214, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t215 * t608);
    let t612 = -t608;
    let t615 = piecewise3::<F>(t219, F::cast_from(0.0_f64), F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t220 * t612);
    let t617 = (t611 + t615) * t43;
    let t622 = t238 * t238;
    let t623 = F::cast_from(1.0_f64) / t622;
    let t624 = t233 * t623;
    let t629 = -F::cast_from(0.1176575e1_f64) * t345 - F::cast_from(0.516475e0_f64) * t347 - F::cast_from(0.2103875e0_f64) * t351 - F::cast_from(0.104195e0_f64) * t353;
    (t617, t624, t629)
}
