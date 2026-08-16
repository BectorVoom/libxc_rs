//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 817/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk817(t3: f64, t8110: f64, t1458: f64, t577: f64, t7423: f64, t7768: f64, t7771: f64, t7773: f64, t2018: f64, t3701: f64, t590: f64, t60: f64) -> (f64, f64, f64, f64) {
    let t8111 = t3 * t8110;
    let t8119 = 0.45e1_f64 * t8110 * t577 + 0.135e2_f64 * t7423 * t1458 + t7768 + t7771 + t7773;
    let t8643 = t3701 * t2018;
    let t8705 = 1.0_f64 / t60 / t590;
    (t8111, t8119, t8643, t8705)
}
