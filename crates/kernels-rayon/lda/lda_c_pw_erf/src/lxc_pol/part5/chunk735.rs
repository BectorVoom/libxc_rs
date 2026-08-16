//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 735/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk735(t173: f64, t6629: f64, t184: f64, t199: f64, t2412: f64, t325: f64, t3633: f64, t6384: f64, t11: f64, t1349: f64, t6379: f64, t6388: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6630 = t173 * t6629;
    let t6631 = t6630 * t184;
    let t6633 = 2.0_f64 / 15.0_f64 * t6631 * t199;
    let t6638 = t325 * t2412;
    let t6640 = t3633 * t6384;
    let t6641 = t11 * t6640;
    let t6643 = t1349 * t6379;
    let t6644 = t11 * t6643;
    let t6646 = t1349 * t6388;
    (t6630, t6631, t6633, t6638, t6640, t6641, t6643, t6644, t6646)
}
