//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 471/1267 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk471(t187: f64, t856: f64, t1550: f64, t1557: f64, t1645: f64, t1646: f64, t2041: f64, t2045: f64, t2068: f64, t2070: f64, t2092: f64, t2097: f64, t2099: f64, t2103: f64, t2105: f64, t2110: f64, t2111: f64, t2113: f64) -> (f64, f64) {
    let t2356 = t856 * t187;
    let t2358 = -t1550 - t2041 - t2045 - t2068 - t2070 - t2092 - t2097 - t2099 - t2103 - t2105 - t2110 + t2111 - t1557 - t2113 + t1645 + 4.0_f64 / 3.0_f64 * t1646 + 4.0_f64 / 3.0_f64 * t2356;
    (t2356, t2358)
}
