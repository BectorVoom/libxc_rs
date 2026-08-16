//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 651/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk651(t5175: f64, t5177: f64, t3436: f64, t380: f64, t251: f64, t3346: f64, t4813: f64) -> (f64, f64, f64, f64) {
    let t5178 = t5175 * t5177;
    let t5180 = t380 * t3436;
    let t5181 = t251 * t3346;
    let t5182 = t5181 * t4813;
    (t5178, t5180, t5181, t5182)
}
