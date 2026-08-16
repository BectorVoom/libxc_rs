//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 777/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk777(t1672: f64, t563: f64, t561: f64, t1: f64, t1952: f64, t119: f64, t713: f64, t1472: f64, t168: f64, t738: f64, t1457: f64, t242: f64) -> (f64, f64, f64, f64) {
    let t5556 = t1672 * t563;
    let t5557 = t561 * t5556;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    let t5562 = 0.15154381759259259259e-2_f64 * t5559 * t5560;
    let t5574 = t168 * t1472 * t738;
    let t5582 = t1457 * t242;
    (t5557, t5562, t5574, t5582)
}
