//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 977/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk977(t1227: f64, t315: f64, t934: f64, t3566: f64, t3576: f64, t28: f64, t3: f64, t37: f64, t27: f64, t4238: f64, t55: f64, t3502: f64, param_hyb_omega_0: f64) -> (f64, f64, f64, f64, f64) {
    let t8323 = t934 * t315 * t1227;
    let t8324 = t3566 * t8323;
    let t8328 = t3576 * t8323;
    let t8333 = 1.0_f64 / t37 / t28 / t3 / 48.0_f64;
    let t8337 = t4238 * t27 * t55;
    let t8339 = 1.6239027777777777_f64 * param_hyb_omega_0 * t8333 * t3502 * t8337;
    (t8324, t8328, t8333, t8337, t8339)
}
