//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 882/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk882(t12384: f64, t1336: f64, t1995: f64, t67: f64, t246: f64, t3700: f64, t570: f64, t1406: f64, t2239: f64, t1454: f64, t2281: f64, t1472: f64, t2517: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12385 = t1336 * t12384;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12461 = 1.0_f64 / t3700 / t570;
    let t12571 = t1406 * t2239;
    let t12747 = t2281 * t1454;
    let t12861 = t1472 * t2517;
    (t12385, t12419, t12461, t12571, t12747, t12861)
}
