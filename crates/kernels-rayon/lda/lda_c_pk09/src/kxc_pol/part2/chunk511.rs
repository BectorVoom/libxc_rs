//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 511/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk511(t556: f64, t1: f64, t2134: f64, t5: f64, t2: f64, t258: f64, t263: f64, t3: f64, t142: f64, t92: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2958 = t556 / 4.0_f64;
    let t2959 = t1 * t2134;
    let t2961 = 2.0_f64 * t5;
    let t2962 = t2 * t258;
    let t2964 = t3 * t263;
    let t2965 = 6.0_f64 * t2964;
    let t2971 = t142 * t92;
    (t2958, t2959, t2961, t2962, t2964, t2965, t2971)
}
