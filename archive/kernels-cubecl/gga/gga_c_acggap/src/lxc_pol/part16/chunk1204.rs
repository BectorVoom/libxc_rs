//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1204/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1204<F: Float>(t13299: F, t33952: F, t40440: F, t15386: F, t34823: F, t40066: F, t31879: F, t36381: F, t36383: F, t36386: F, t36389: F, t36391: F, t37994: F, t40553: F, t40557: F, t40561: F, t40565: F, t40567: F, t40569: F, t40573: F, t40576: F, t40579: F) -> F {
    let t40584 = t33952 * t13299 * t40440;
    let t40587 = t34823 * t15386 * t40066;
    let t40589 = F::cast_from(0.18868855373762491241e-2_f64) * t40553 + F::cast_from(0.18868855373762491241e-2_f64) * t40557 + F::cast_from(0.18868855373762491241e-2_f64) * t40561 + F::cast_from(0.12579236915841660827e-2_f64) * t40565 + F::cast_from(0.18868855373762491241e-2_f64) * t40567 - F::cast_from(0.916875e-1_f64) * t40569 + t36381 - t40573 / F::cast_from(384.0_f64) + t36383 + F::cast_from(0.7640625e-2_f64) * t40576 + t40579 / F::cast_from(32.0_f64) - F::cast_from(0.85748036236139473944e-3_f64) * t31879 + F::cast_from(0.5590771962596293701e-2_f64) * t36386 + t36389 + t36391 + t37994 + F::cast_from(0.31448092289604152068e-2_f64) * t40584 + F::cast_from(0.18868855373762491241e-2_f64) * t40587;
    t40589
}
