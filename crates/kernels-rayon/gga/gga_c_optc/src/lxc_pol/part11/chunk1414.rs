//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1414/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1414(t5122: f64, t33724: f64, t33730: f64, t43414: f64, t44193: f64, t44198: f64, t52389: f64, t52391: f64, t52393: f64, t58348: f64, t58352: f64, t58356: f64, t58360: f64, t58363: f64, t58367: f64) -> (f64, f64) {
    let t59263 = t5122 * t5122;
    let t59281 = -0.23154444444444444445e0_f64 * t44193 + 0.13892666666666666667e1_f64 * t44198 - 0.91817777777777777776e0_f64 * t43414 + 0.21424148148148148148e1_f64 * t33724 + 0.12349037037037037037e1_f64 * t33730 + 0.41318e1_f64 * t58348 + 0.125034e1_f64 * t58352 - 0.27785333333333333334e0_f64 * t58356 - 0.375102e1_f64 * t58360 + 0.83356e0_f64 * t58363 - 0.13892666666666666667e0_f64 * t58367 + 0.68863333333333333332e0_f64 * t52389 + 0.41318e1_f64 * t52391 + 0.76514814814814814814e0_f64 * t52393;
    (t59263, t59281)
}
