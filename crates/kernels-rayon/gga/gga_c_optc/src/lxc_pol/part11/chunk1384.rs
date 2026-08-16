//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1384/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1384(t33724: f64, t33730: f64, t43414: f64, t44193: f64, t44198: f64, t52389: f64, t52391: f64, t52393: f64, t58348: f64, t58352: f64, t58356: f64, t58360: f64, t58363: f64, t58367: f64) -> f64 {
    let t58701 = -0.18396666666666666667e0_f64 * t44193 + 0.11038e1_f64 * t44198 - 0.53675555555555555556e0_f64 * t43414 + 0.12524296296296296297e1_f64 * t33724 + 0.98115555555555555556e0_f64 * t33730 + 0.24154e1_f64 * t58348 + 0.99342e0_f64 * t58352 - 0.22076e0_f64 * t58356 - 0.298026e1_f64 * t58360 + 0.66228e0_f64 * t58363 - 0.11038e0_f64 * t58367 + 0.40256666666666666668e0_f64 * t52389 + 0.24154e1_f64 * t52391 + 0.44729629629629629629e0_f64 * t52393;
    t58701
}
