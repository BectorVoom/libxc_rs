//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1130/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1130(t17617: f64, t1893: f64, t5077: f64, t1864: f64, t2630: f64, t1859: f64, t5083: f64, t15862: f64, t6562: f64, t6630: f64, t15865: f64, t6633: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t20569 = 4.0_f64 / 15.0_f64 * t5077 * t17617 * t1893;
    let t20572 = 4.0_f64 / 15.0_f64 * t5077 * t2630 * t1864;
    let t20575 = 2.0_f64 / 9.0_f64 * t5083 * t2630 * t1859;
    let t20577 = 4.0_f64 / 15.0_f64 * t15862 * t6562;
    let t20579 = 4.0_f64 / 15.0_f64 * t15862 * t6630;
    let t20581 = 2.0_f64 / 9.0_f64 * t15865 * t6633;
    (t20569, t20572, t20575, t20577, t20579, t20581)
}
