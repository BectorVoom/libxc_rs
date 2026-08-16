//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1042/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1042(t1777: f64, t2758: f64, t11248: f64, t1823: f64, t1672: f64, t2877: f64, t1859: f64, t1468: f64, t2777: f64, t1782: f64, t1838: f64, t1841: f64, t2778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11299 = t1777 * t2758;
    let t11302 = t1823 * t11248;
    let t11304 = t2877 * t1672;
    let t11306 = t1859 * t11248;
    let t11310 = t2777 * t1468;
    let t11311 = t11310 * t1782;
    let t11312 = t11311 * t1838;
    let t11314 = t2778 * t1841;
    (t11299, t11302, t11304, t11306, t11311, t11312, t11314)
}
