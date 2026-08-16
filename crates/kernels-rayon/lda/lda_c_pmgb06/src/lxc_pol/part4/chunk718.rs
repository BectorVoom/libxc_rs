//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 718/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk718(t4529: f64, t693: f64, t1112: f64, t2151: f64, t2160: f64, t643: f64, t2158: f64, t638: f64, t248: f64, t3662: f64, t3672: f64, t3678: f64, t3700: f64, t4483: f64, t4485: f64, t4516: f64, t4518: f64, t4520: f64, t4522: f64, t4525: f64, t4527: f64) -> (f64, f64, f64, f64, f64) {
    let t4531 = 0.0003662289461201309_f64 * t4529 * t693;
    let t4532 = t2151 * t1112;
    let t4534 = t643 * t2160;
    let t4537 = 8.0_f64 * t638 * t2158;
    let t4538 = t4483 - t4485 + t248 * t4516 + 8.0_f64 * t4518 + 12.0_f64 * t4520 + 20.0_f64 * t4522 + t4525 + 0.0004883052614935079_f64 * t3662 - 32.0_f64 * t4527 + t3672 - t3678 + t3700 - t4531 + 0.00024415263074675396_f64 * t4532 - 8.0_f64 * t4534 + t4537;
    (t4531, t4532, t4534, t4537, t4538)
}
