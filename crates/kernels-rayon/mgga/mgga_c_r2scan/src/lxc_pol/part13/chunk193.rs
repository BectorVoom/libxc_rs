//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 193/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk193(t495: f64, t551: f64, t552: f64, t239: f64, t378: f64, t5: f64, t152: f64, t153: f64, t158: f64) -> (f64, f64, f64, f64) {
    let t576 = t551 * t552 * t495;
    let t581 = 5.0_f64 / 3.0_f64 * t5 * t378 * t239;
    let t583 = 1.0_f64 / t153 / t152;
    let t584 = t583 * t158;
    (t576, t581, t583, t584)
}
