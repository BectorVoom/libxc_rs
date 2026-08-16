//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1795/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1795(t23218: f64, t6553: f64, t1880: f64, t2553: f64, t6554: f64, t6552: f64, t218: f64, t23150: f64, t212: f64, t252: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23219 = t6553 * t23218;
    let t23220 = t1880 * t23219;
    let t23222 = t6554 * t2553;
    let t23223 = t6553 * t23222;
    let t23224 = t6552 * t23223;
    let t23226 = t218 * t23150;
    let t23228 = t212 * t252;
    (t23219, t23220, t23222, t23223, t23224, t23226, t23228)
}
