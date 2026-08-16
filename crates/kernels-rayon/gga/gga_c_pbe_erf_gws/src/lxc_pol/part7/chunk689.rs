//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 689/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk689(t5548: f64, t5550: f64, t587: f64, t1868: f64, t579: f64, t1672: f64, t563: f64, t561: f64, t1: f64, t1952: f64, t119: f64, t713: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5551 = t5548 * t5550;
    let t5553 = 8.0_f64 / 15.0_f64 * t587 * t5551;
    let t5555 = 2.0_f64 / 5.0_f64 * t579 * t1868;
    let t5556 = t1672 * t563;
    let t5557 = t561 * t5556;
    let t5558 = 8.0_f64 / 45.0_f64 * t5557;
    let t5559 = t1952 * t1;
    let t5560 = t119 * t713;
    (t5551, t5553, t5555, t5556, t5558, t5559, t5560)
}
