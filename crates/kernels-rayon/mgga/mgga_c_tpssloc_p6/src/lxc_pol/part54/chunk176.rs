//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 176/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk176(t563: f64, t568: f64, t144: f64, t193: f64, t523: f64, t525: f64, t533: f64) -> (f64, f64, f64) {
    let t570 = t563 * t568 + 1.0_f64;
    let t571 = f64::ln(t570);
    let t574 = t193 * t533 * t571 - t144 + t523 + t525;
    (t570, t571, t574)
}
