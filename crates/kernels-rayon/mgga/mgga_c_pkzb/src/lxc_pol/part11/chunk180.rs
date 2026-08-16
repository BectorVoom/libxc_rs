//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 180/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk180(t12: f64, t24: f64, t207: f64, t439: f64, t333: f64, t507: f64, zeta_threshold: f64) -> f64 {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t563 = piecewise3(t84, 0.0_f64, 2.0_f64 / 3.0_f64 * t207 * t439);
    let t566 = piecewise3(t90, 0.0_f64, 2.0_f64 / 3.0_f64 * t333 * t507);
    let t568 = t563 / 2.0_f64 + t566 / 2.0_f64;
    t568
}
