//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1280/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1280(t13090: f64, t13092: f64, t16797: f64, t16800: f64, t16801: f64, t16806: f64, t16809: f64, t16812: f64, t16817: f64, t16820: f64, t16824: f64, t16828: f64, t16833: f64, t16835: f64) -> (f64, f64, f64) {
    let t16836 = 8.0_f64 / 405.0_f64 * t13090;
    let t16837 = 2.0_f64 / 45.0_f64 * t13092;
    let t16838 = t16797 + t16800 + t16801 + t16806 - t16809 - t16812 - t16817 + t16820 + t16824 + t16828 + t16833 - t16835 - t16836 - t16837;
    (t16836, t16837, t16838)
}
