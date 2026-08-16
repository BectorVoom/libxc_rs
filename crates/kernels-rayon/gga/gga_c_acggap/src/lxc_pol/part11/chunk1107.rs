//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1107/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1107(t35194: f64, t1165: f64, t21118: f64, t7426: f64, t8600: f64, t5209: f64, t7822: f64, t30990: f64, t30991: f64, t30998: f64, t31002: f64, t31003: f64, t31016: f64, t31021: f64, t31023: f64, t35167: f64, t35172: f64, t35176: f64, t35180: f64, t35184: f64, t35186: f64, t35191: f64) -> f64 {
    let t35195 = 0.18868855373762491241e-2_f64 * t35194;
    let t35198 = t7426 * t1165 * t8600 * t21118;
    let t35199 = 0.37737710747524982482e-2_f64 * t35198;
    let t35200 = t7822 * t5209;
    let t35202 = -t30990 - 0.64311027177104605458e-3_f64 * t30991 - t35167 - 0.20965394859736101378e-3_f64 * t30998 + t31002 - 0.85748036236139473944e-3_f64 * t31003 + t31016 - t31021 + t31023 - 0.62896184579208304136e-3_f64 * t35172 - 0.41930789719472202757e-3_f64 * t35176 + 0.10718504529517434243e-3_f64 * t35180 - 0.20965394859736101378e-3_f64 * t35184 - 0.64311027177104605458e-2_f64 * t35186 + t35191 - t35195 + t35199 + 0.34299214494455789578e-2_f64 * t35200;
    t35202
}
