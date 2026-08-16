//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1204/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1204(t13299: f64, t33952: f64, t40440: f64, t15386: f64, t34823: f64, t40066: f64, t31879: f64, t36381: f64, t36383: f64, t36386: f64, t36389: f64, t36391: f64, t37994: f64, t40553: f64, t40557: f64, t40561: f64, t40565: f64, t40567: f64, t40569: f64, t40573: f64, t40576: f64, t40579: f64) -> f64 {
    let t40584 = t33952 * t13299 * t40440;
    let t40587 = t34823 * t15386 * t40066;
    let t40589 = 0.18868855373762491241e-2_f64 * t40553 + 0.18868855373762491241e-2_f64 * t40557 + 0.18868855373762491241e-2_f64 * t40561 + 0.12579236915841660827e-2_f64 * t40565 + 0.18868855373762491241e-2_f64 * t40567 - 0.916875e-1_f64 * t40569 + t36381 - t40573 / 384.0_f64 + t36383 + 0.7640625e-2_f64 * t40576 + t40579 / 32.0_f64 - 0.85748036236139473944e-3_f64 * t31879 + 0.5590771962596293701e-2_f64 * t36386 + t36389 + t36391 + t37994 + 0.31448092289604152068e-2_f64 * t40584 + 0.18868855373762491241e-2_f64 * t40587;
    t40589
}
