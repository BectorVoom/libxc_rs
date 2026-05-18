//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 793/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk793<F: Float>(t2046: F, t8589: F, t7316: F, t7318: F, t8556: F, t8558: F, t8562: F, t8567: F, t8572: F, t8574: F, t8576: F, t8578: F, t8580: F, t8582: F, t8584: F, t8586: F) -> F {
    let t8590 = t2046 * t8589;
    let t8594 = F::new(0.52413487149340253447e-3) * t8556 - F::new(0.31448092289604152068e-3) * t8558 - F::new(0.31448092289604152068e-3) * t8562 - F::new(0.31448092289604152068e-3) * t8567 - F::new(0.20965394859736101379e-3) * t8572 - F::new(0.42874018118069736972e-3) * t8574 - F::new(0.42874018118069736972e-3) * t8576 + F::new(0.47172138434406228102e-3) * t8578 - F::new(0.94344276868812456204e-3) * t8580 - F::new(0.10718504529517434243e-3) * t8582 - t8584 / F::new(96.0) - t8586 / F::new(96.0) - t8590 / F::new(128.0) + F::new(11.0) / F::new(384.0) * t7316 + F::new(11.0) / F::new(1152.0) * t7318;
    t8594
}
