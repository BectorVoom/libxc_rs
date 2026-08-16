//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1160/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1160(t12012: f64, t2496: f64, t493: f64, t5486: f64, t6390: f64, t1981: f64, t6394: f64, t6755: f64, t6747: f64, t6760: f64, t20919: f64, t20920: f64, t20922: f64, t20925: f64, t20929: f64, t20931: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20934 = 2.0_f64 / 15.0_f64 * t493 * t12012 * t2496;
    let t20937 = 2.0_f64 / 15.0_f64 * t493 * t5486 * t6390;
    let t20940 = 4.0_f64 / 15.0_f64 * t1981 * t5486 * t6394;
    let t20943 = t493 * t5486 * t6755 / 15.0_f64;
    let t20946 = 2.0_f64 / 15.0_f64 * t493 * t6747 * t6760;
    let t20947 = -t20919 + t20920 - t20922 - t20925 - t20929 - t20931 - t20934 - t20937 + t20940 - t20943 - t20946;
    (t20934, t20937, t20940, t20943, t20946, t20947)
}
