//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1396/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1396(t16100: f64, t16101: f64, t16102: f64, t16103: f64, t16105: f64, t16107: f64, t16109: f64, t16112: f64, t16114: f64, t16117: f64, t16121: f64, t16122: f64, t9424: f64, t9426: f64, t9429: f64) -> f64 {
    let t18206 = (4e-21_f64 as f64) * t9424 + 16.0_f64 / 81.0_f64 * t9426 + t9429 - t16100 + t16101 - t16102 - t16103 - t16105 - t16107 + t16109 + t16112 + t16114 + t16117 + t16121 + t16122;
    t18206
}
