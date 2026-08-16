//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 662/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk662(t280: f64, t39: f64, t2035: f64, t1196: f64, t817: f64, t800: f64, t4092: f64, t10363: f64, t5284: f64, t5260: f64, t816: f64, t291: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19038 = t280 * t39;
    let t19039 = t19038 * t2035;
    let t19048 = t817 * t1196;
    let t19049 = t800 * t19048;
    let t19053 = t4092 * t19048;
    let t19080 = t10363 * t5284;
    let t19095 = t816 * t5260;
    let t19100 = t291 * t39;
    (t19038, t19039, t19049, t19053, t19080, t19095, t19100)
}
