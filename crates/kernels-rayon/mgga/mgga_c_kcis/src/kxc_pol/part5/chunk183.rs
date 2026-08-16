//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 183/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk183(t553: f64, t557: f64, t303: f64, t486: f64, t507: f64, t143: f64) -> (f64, f64, f64, f64) {
    let t558 = t553 * t557;
    let t559 = t303 * t558;
    let t561 = t486 * t507 + 0.24872916666666666666e-2_f64 * t559;
    let t562 = t486 * t143;
    (t558, t559, t561, t562)
}
