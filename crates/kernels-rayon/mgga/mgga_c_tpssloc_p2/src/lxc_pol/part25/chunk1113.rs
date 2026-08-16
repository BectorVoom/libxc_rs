//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1113/1226 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1113(t22734: f64, t81159: f64, t1352: f64, t26331: f64, t3734: f64, t562: f64, t6976: f64, t22633: f64, t81052: f64, t1992: f64, t22897: f64, t3792: f64, t81094: f64) -> (f64, f64, f64, f64) {
    let t81160 = t81159 * t22734;
    let t81165 = t26331 * t6976 * t562 * t3734 * t1352;
    let t81169 = t22633 * t6976 * t81052 * t1352;
    let t81173 = t1992 * t22897 * t81094 * t3792;
    (t81160, t81165, t81169, t81173)
}
