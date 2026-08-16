//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 967/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk967(t1423: f64, t6472: f64, t5211: f64, t6382: f64, t436: f64, t6705: f64, t1517: f64, t2592: f64, t161: f64, t489: f64, t6231: f64, t5499: f64, t6536: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15772 = t1423 * t6472;
    let t15774 = t5211 * t6382;
    let t15793 = t6705 * t436;
    let t15795 = t2592 * t1517;
    let t15807 = t161 * t489 * t6231;
    let t15829 = t5499 * t6536;
    (t15772, t15774, t15793, t15795, t15807, t15829)
}
