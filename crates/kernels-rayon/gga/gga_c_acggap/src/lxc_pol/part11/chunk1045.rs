//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1045/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1045(t1967: f64, t8561: f64, t30340: f64, t30349: f64, t30353: f64, t30355: f64, t30362: f64, t34327: f64, t34330: f64, t34332: f64, t34333: f64, t34336: f64, t34338: f64, t34339: f64, t34341: f64, t34343: f64, t34348: f64, t34349: f64) -> f64 {
    let t34351 = t1967 * t8561;
    let t34352 = 0.37737710747524982482e-2_f64 * t34351;
    let t34357 = 0.114609375e-1_f64 * t34327 + 0.22921875e-1_f64 * t34330 - t34332 - t34333 + t30340 + 0.31448092289604152068e-3_f64 * t34336 + t34338 + t34339 + t34341 - 0.10718504529517434243e-3_f64 * t34343 - t34348 - 0.37737710747524982482e-2_f64 * t34349 + t34352 - 0.94344276868812456204e-3_f64 * t30349 + 0.10482697429868050689e-3_f64 * t30353 - 0.10718504529517434243e-3_f64 * t30355 - 0.7145669686344956162e-4_f64 * t30362;
    t34357
}
