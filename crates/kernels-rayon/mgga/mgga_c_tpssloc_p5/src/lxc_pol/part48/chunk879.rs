//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 879/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk879(t5: f64, t31857: f64, t31860: f64, t31864: f64, t31868: f64, t32328: f64, t32333: f64, t32340: f64, t32346: f64, t8663: f64, t8825: f64, t112: f64, t111: f64, t8828: f64) -> (f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t32348 = piecewise3(t8, 0.0_f64, -5.0_f64 / 72.0_f64 * t31857 * t8825 + 5.0_f64 / 12.0_f64 * t31860 * t32328 + 5.0_f64 / 18.0_f64 * t31864 * t32333 - 5.0_f64 / 72.0_f64 * t31868 * t8825 - 5.0_f64 / 36.0_f64 * t8663 * t32340 + t32346);
    let t32349 = t32348 * t112;
    let t32350 = t8828 * t111;
    (t32348, t32349, t32350)
}
