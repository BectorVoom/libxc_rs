//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 901/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk901(t1609: f64, t1610: f64, t1613: f64, t1593: f64, t1597: f64, t62: f64, t66: f64, t11240: f64, t371: f64, t19: f64, t7: f64, t11: f64, t1690: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37481 = t1613 * t1610 * t1609;
    let t37482 = t37481 * t1593;
    let t37939 = t1597 * t62;
    let t37940 = t37939 * t66;
    let t37985 = t371 * t11240;
    let t37991 = t7 * t19;
    let t38176 = t1690 * t11;
    (t37481, t37482, t37939, t37940, t37985, t37991, t38176)
}
