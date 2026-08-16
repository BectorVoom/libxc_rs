//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 947/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk947(t1147: f64, t794: f64, t1193: f64, t1354: f64, t1798: f64, t740: f64, t1186: f64, t421: f64, t5617: f64, t2329: f64, t2837: f64, t1179: f64, t419: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14277 = t1147 * t794;
    let t14279 = t14277 * t1193 * t1354;
    let t14281 = t740 * t1798;
    let t14283 = t14281 * t1193 * t1354;
    let t14284 = 0.0014862827083471494_f64 * t14283;
    let t14290 = t5617 * t1186 * t421;
    let t14291 = 0.01185233419734569_f64 * t14290;
    let t14293 = t2329 * t2837 * t421;
    let t14297 = t1179 * t1798 * t419 * t421;
    (t14277, t14279, t14281, t14284, t14291, t14293, t14297)
}
