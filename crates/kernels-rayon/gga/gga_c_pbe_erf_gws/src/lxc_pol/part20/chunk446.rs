//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 446/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk446(t172: f64, t577: f64, t184: f64, t563: f64, t582: f64, t561: f64, t188: f64, t22: f64) -> (f64, f64, f64, f64, f64) {
    let t1878 = t172 * t577;
    let t1879 = t1878 * t184;
    let t1882 = t582 * t563;
    let t1883 = t561 * t1882;
    let t1885 = t22 * t188;
    (t1878, t1879, t1882, t1883, t1885)
}
