//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 957/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk957(t11386: f64, t2437: f64, t13434: f64, t18651: f64, t11413: f64, t1445: f64, t2293: f64, t4527: f64, t13276: f64, t4540: f64, t4673: f64, t13438: f64, t4953: f64) -> (f64, f64, f64, f64, f64) {
    let t46212 = 0.35750489951850426669e0_f64 * t2437 * t11386;
    let t46216 = 0.27606906686822939767e2_f64 * t18651 * t13434;
    let t46220 = 0.27606906686822939767e2_f64 * t4527 * t1445 * t11413 * t2293;
    let t46223 = 0.14300195980740170667e1_f64 * t4540 * t4673 * t13276;
    let t46225 = 0.69017266717057349418e1_f64 * t4953 * t13438;
    (t46212, t46216, t46220, t46223, t46225)
}
