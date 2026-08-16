//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 942/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk942(t109230: f64, t141111: f64, t1613: f64, t92354: f64, t9533: f64, t218: f64, t41: f64, t2344: f64, t679: f64, t7205: f64, t33432: f64, t3789: f64, t7203: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t141112 = t109230 * t141111;
    let t141116 = t92354 * t1613 * sigma2;
    let t141117 = t9533 * t141116;
    let t141121 = t1613 * t218;
    let t141123 = t9533 * t41 * t141121;
    let t141125 = t7205 * t2344 * t679;
    let t141160 = t3789 * t33432 * t7203;
    (t141112, t141116, t141117, t141123, t141125, t141160)
}
