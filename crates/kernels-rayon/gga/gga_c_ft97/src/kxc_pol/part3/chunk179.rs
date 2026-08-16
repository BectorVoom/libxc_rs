//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 179/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk179(t526: f64, t558: f64, t27: f64, t89: f64, t518: f64, t522: f64, t515: f64, t161: f64, t376: f64, t143: f64, t378: f64) -> (f64, f64, f64, f64, f64) {
    let t559 = t526 * t558;
    let t561 = t89 * t27 * t559;
    let t563 = -t518 - t522 / 18.0_f64 - t561 / 6.0_f64;
    let t564 = t515 * t563;
    let t568 = t89 * t376 * t161 / 9.0_f64;
    let t569 = t378 * t143;
    (t559, t561, t564, t568, t569)
}
