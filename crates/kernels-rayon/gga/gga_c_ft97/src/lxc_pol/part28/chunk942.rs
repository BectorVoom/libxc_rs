//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 942/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk942(t136559: f64, t1624: f64, t2258: f64, t36363: f64, t36390: f64, t5567: f64, t92353: f64, t22696: f64, t32145: f64, t22701: f64, t9: f64, t1669: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t136560 = t1624 * t136559;
    let t136561 = t36363 * t2258;
    let t136565 = t36390 * t5567;
    let t136566 = t92353 * t136565;
    let t136572 = t22696 * t32145;
    let t136575 = t22701 * t9;
    let t136576 = t1669 * t136575;
    (t136560, t136561, t136565, t136566, t136572, t136575, t136576)
}
