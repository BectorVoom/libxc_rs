//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1169/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1169(t1901: f64, t19762: f64, t2010: f64, t1420: f64, t7551: f64, t10148: f64, t439: f64, t7550: f64, t2064: f64, t2570: f64, t2960: f64, t187: f64, t7704: f64) -> (f64, f64, f64, f64, f64) {
    let t21050 = 2.0_f64 / 9.0_f64 * t2010 * t1901 * t19762;
    let t21052 = t1420 * t7551 / 9.0_f64;
    let t21055 = t439 * t10148 * t7550 / 9.0_f64;
    let t21059 = t439 * t2960 * t2570 * t2064 / 9.0_f64;
    let t21061 = t7704 * t187;
    (t21050, t21052, t21055, t21059, t21061)
}
