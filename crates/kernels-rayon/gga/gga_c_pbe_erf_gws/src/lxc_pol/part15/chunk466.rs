//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 466/1352 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk466(t1879: f64, t564: f64, t563: f64, t582: f64, t561: f64, t188: f64, t22: f64) -> (f64, f64, f64, f64, f64) {
    let t1881 = 8.0_f64 / 15.0_f64 * t1879 * t564;
    let t1882 = t582 * t563;
    let t1883 = t561 * t1882;
    let t1884 = 16.0_f64 / 45.0_f64 * t1883;
    let t1885 = t22 * t188;
    (t1881, t1882, t1883, t1884, t1885)
}
