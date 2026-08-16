//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 699/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk699(t11034: f64, t1866: f64, t446: f64, t1588: f64, t7241: f64, t942: f64, t28: f64, t89: f64, t7773: f64, t921: f64, t1570: f64, t1559: f64) -> (f64, f64, f64, f64) {
    let t11035 = t1866 * t11034;
    let t11036 = t446 * t11035;
    let t11039 = t7241 * t942 * t1588;
    let t11041 = t89 * t28 * t11039;
    let t11043 = t89 * t7773 * t921;
    let t11045 = t942 * t1570;
    let t11046 = t11045 * t1559;
    (t11036, t11041, t11043, t11046)
}
