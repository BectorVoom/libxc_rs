//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 201/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk201(t198: f64, t562: f64, t186: f64, t561: f64, t155: f64, t56: f64, t174: f64, t177: f64, t188: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t563 = t198 * t562;
    let t564 = t186 * t563;
    let t566 = 4.0_f64 / 15.0_f64 * t561 * t564;
    let t567 = t155 * t56;
    let t569 = t174 * t567 * t177;
    let t570 = 0.18891666666666666667e-2_f64 * t569;
    let t571 = t56 * t188;
    (t563, t564, t566, t567, t569, t570, t571)
}
