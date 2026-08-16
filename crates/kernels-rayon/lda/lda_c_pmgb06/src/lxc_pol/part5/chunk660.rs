//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 660/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk660(t2236: f64, t73: f64, t2432: f64, t707: f64, t23: f64, t342: f64, t2377: f64, t3537: f64, t1212: f64, t2381: f64, t4433: f64, t4434: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5934 = t73 * t2236;
    let t5937 = t707 * t2432;
    let t5939 = t342 * t23;
    let t5953 = t3537 * t2377;
    let t5958 = t1212 * t2381;
    let t5961 = -t4433 - t4434;
    (t5934, t5937, t5939, t5953, t5958, t5961)
}
