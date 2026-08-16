//! LDA_C_PMGB06 lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1140/1478 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part4_v4rho4_2_chunk1140(t27: f64, t6067: f64, t693: f64, t1112: f64, t6078: f64, t248: f64, t686: f64, t14935: f64, t285: f64, t8590: f64, t8594: f64, t8598: f64, t8603: f64, t8610: f64, t8612: f64, t8614: f64, t8616: f64, t8621: f64, t8626: f64, t8629: f64, t8633: f64, t8637: f64) -> f64 {
    let t14971 = t6067 * t27 * t693;
    let t14973 = t6078 * t1112;
    let t14977 = t248 * t6067 * t686;
    let t14981 = -24.0_f64 * t8590 - t8594 - t8598 + t8603 + t8610 - t8612 - 160.0_f64 * t8614 - 0.0003662289461201309_f64 * t14971 + 0.00024415263074675396_f64 * t14973 + 20.0_f64 * t8616 + t8621 - t8626 + 2.0_f64 * t14977 + t248 * t14935 * t285 - t8629 - t8633 - t8637;
    t14981
}
