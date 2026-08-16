//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1144/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1144(t34927: f64, t376: f64, t89: f64, t148132: f64, t32906: f64, t7239: f64, t7366: f64, t3526: f64, t7312: f64, t32888: f64, t7369: f64, t147656: f64, t446: f64, t9073: f64) -> (f64, f64, f64, f64, f64) {
    let t148511 = t89 * t376 * t34927;
    let t148515 = t7366 * t7239 * t32906 * t148132;
    let t148517 = t7312 * t3526;
    let t148520 = t32888 * t7239 * t7369 * t148517;
    let t148523 = t446 * t9073 * t147656;
    (t148511, t148515, t148517, t148520, t148523)
}
