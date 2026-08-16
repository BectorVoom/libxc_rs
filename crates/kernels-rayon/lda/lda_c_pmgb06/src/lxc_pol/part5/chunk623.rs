//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 623/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk623(t2085: f64, t4913: f64, t1832: f64, t4641: f64, t2094: f64, t489: f64, t161: f64, t1636: f64, t831: f64, t4637: f64, t819: f64, t955: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4914 = t4913 * t2085;
    let t4916 = t4641 * t1832;
    let t4948 = t489 * t2094;
    let t4950 = 2.0_f64 / 45.0_f64 * t161 * t4948;
    let t4970 = 2.0_f64 / 45.0_f64 * t831 * t1636;
    let t5002 = 0.015996296296296297_f64 * t4637;
    let t5003 = t955 * t819;
    (t4914, t4916, t4948, t4950, t4970, t5002, t5003)
}
