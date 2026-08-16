//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 779/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk779(t7464: f64, t7466: f64, t7468: f64, t7479: f64, t7481: f64, t7484: f64, t7488: f64, t7496: f64, t7500: f64, t7516: f64, t7520: f64, t8740: f64, t8742: f64, t8744: f64, t8748: f64) -> f64 {
    let t8750 = 0.18868855373762491241e-2_f64 * t7464 - 0.28303283060643736861e-2_f64 * t7466 + 0.7862023072401038017e-3_f64 * t7468 + 0.52413487149340253445e-3_f64 * t7479 - 0.31448092289604152068e-3_f64 * t7481 + 0.22921875e-1_f64 * t7484 + 0.1528125e-1_f64 * t7488 - 0.7862023072401038017e-3_f64 * t7496 + 0.31448092289604152068e-3_f64 * t7500 + 0.31448092289604152068e-3_f64 * t8740 + 0.22921875e-1_f64 * t8742 + 0.1528125e-1_f64 * t8744 + t7516 - t7520 - 0.38203125e-2_f64 * t8748;
    t8750
}
