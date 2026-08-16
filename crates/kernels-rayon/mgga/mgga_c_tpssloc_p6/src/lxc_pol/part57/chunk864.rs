//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 864/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk864(t1459: f64, t1849: f64, t2040: f64, t31532: f64, t33085: f64, t33199: f64, t33204: f64, t33208: f64, t33213: f64, t33216: f64, t33218: f64, t33224: f64, t33227: f64, t6517: f64, t652: f64, t7042: f64, t7472: f64, t7802: f64, t8604: f64) -> f64 {
    let t33228 = -2.0_f64 * t1459 * t31532 + t1849 * t8604 - 2.0_f64 * t2040 * t33085 - 2.0_f64 * t33204 * t652 - 2.0_f64 * t6517 * t7802 - 2.0_f64 * t7042 * t7472 - t33199 - t33208 - t33213 - t33216 - t33218 + t33224 - t33227;
    t33228
}
