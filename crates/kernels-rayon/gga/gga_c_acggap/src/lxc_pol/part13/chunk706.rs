//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 706/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk706(t2100: f64, t7538: f64, t7463: f64, t7465: f64, t7466: f64, t7469: f64, t7473: f64, t7479: f64, t7481: f64, t7485: f64, t7489: f64, t7491: f64, t7497: f64, t7500: f64, t7504: f64, t7516: f64, t7520: f64, t7524: f64, t7529: f64, t7531: f64, t7536: f64) -> (f64, f64) {
    let t7539 = t7538 * t2100;
    let t7540 = 0.31448092289604152068e-3_f64 * t7539;
    let t7541 = -t7463 + t7465 - 0.56606566121287473722e-2_f64 * t7466 + t7469 + 0.7862023072401038017e-3_f64 * t7473 + 0.10482697429868050689e-2_f64 * t7479 - 0.62896184579208304136e-3_f64 * t7481 + t7485 + t7489 + 0.1528125e-1_f64 * t7491 - t7497 + 0.62896184579208304136e-3_f64 * t7500 - 0.4584375e-1_f64 * t7504 + t7516 - t7520 - 0.31448092289604152068e-3_f64 * t7524 - 0.41930789719472202757e-3_f64 * t7529 + 0.94344276868812456204e-3_f64 * t7531 + 0.47172138434406228102e-3_f64 * t7536 + t7540;
    (t7540, t7541)
}
