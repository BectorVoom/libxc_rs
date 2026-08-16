//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 973/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk973(t2347: f64, t7584: f64, t1486: f64, t33874: f64, t681: f64, t1636: f64, t7654: f64, t89: f64, t141365: f64, t7638: f64, t7642: f64, t33288: f64, t33811: f64, t33813: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t143239 = t7584 * t2347;
    let t143245 = t1486 * t681 * t33874;
    let t143263 = t89 * t1636 * t7654;
    let t143264 = 8.0_f64 / 27.0_f64 * t143263;
    let t143273 = t7638 * t141365 * t7642;
    let t143274 = 10.0_f64 / 27.0_f64 * t143273;
    let t143276 = t33811 * t33288 * t33813;
    (t143239, t143245, t143263, t143264, t143273, t143274, t143276)
}
