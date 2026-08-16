//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 603/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk603(t1152: f64, t123: f64, t566: f64, t290: f64, t642: f64, t247: f64, t701: f64, t2789: f64, t301: f64, t83: f64, t297: f64, t4001: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4257 = t123 * t1152 * t566;
    let t4283 = 1.279801625812305_f64 * t642 * t290;
    let t4284 = t247 * t701;
    let t4294 = t83 * t2789 * t301;
    let t4296 = 0.01197423401025461_f64 * t297 * t4294;
    let t4297 = t4001 * t83;
    (t4257, t4283, t4284, t4294, t4296, t4297)
}
