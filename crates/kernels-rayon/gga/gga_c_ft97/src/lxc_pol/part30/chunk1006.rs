//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1006/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1006(t33292: f64, t33294: f64, t36867: f64, t3837: f64, t898: f64, t140756: f64, t140757: f64, t27836: f64, t33288: f64, t35324: f64, t7511: f64, t1154: f64, t193: f64, t33452: f64, t6109: f64, t743: f64) -> (f64, f64, f64, f64) {
    let t150184 = t33292 * t898 * t36867 * t33294 * t3837;
    let t150188 = t33292 * t140756 * t140757 * t27836;
    let t150194 = t7511 * t33288 * t35324;
    let t150199 = t6109 * t193 * t743 * t33452 * t1154;
    (t150184, t150188, t150194, t150199)
}
