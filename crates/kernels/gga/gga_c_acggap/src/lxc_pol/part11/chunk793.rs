//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 793/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk793<F: Float>(t2046: F, t8589: F, t7316: F, t7318: F, t8556: F, t8558: F, t8562: F, t8567: F, t8572: F, t8574: F, t8576: F, t8578: F, t8580: F, t8582: F, t8584: F, t8586: F) -> F {
    let t8590 = t2046 * t8589;
    let t8594 = F::cast_from(0.52413487149340253447e-3_f64) * t8556 - F::cast_from(0.31448092289604152068e-3_f64) * t8558 - F::cast_from(0.31448092289604152068e-3_f64) * t8562 - F::cast_from(0.31448092289604152068e-3_f64) * t8567 - F::cast_from(0.20965394859736101379e-3_f64) * t8572 - F::cast_from(0.42874018118069736972e-3_f64) * t8574 - F::cast_from(0.42874018118069736972e-3_f64) * t8576 + F::cast_from(0.47172138434406228102e-3_f64) * t8578 - F::cast_from(0.94344276868812456204e-3_f64) * t8580 - F::cast_from(0.10718504529517434243e-3_f64) * t8582 - t8584 / F::cast_from(96.0_f64) - t8586 / F::cast_from(96.0_f64) - t8590 / F::cast_from(128.0_f64) + F::cast_from(11.0_f64) / F::cast_from(384.0_f64) * t7316 + F::cast_from(11.0_f64) / F::cast_from(1152.0_f64) * t7318;
    t8594
}
