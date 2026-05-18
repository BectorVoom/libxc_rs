//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 504/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk504<F: Float>(t464: F, t813: F, t477: F, t137: F, t132: F, t1552: F, t1637: F, t1550: F, t1557: F, t1708: F, t1712: F, t1732: F, t2039: F, t2041: F, t2045: F, t2068: F, t2070: F, t2092: F, t2097: F, t2099: F, t2103: F, t2105: F) -> (F, F, F, F, F, F, F) {
    let t2106 = t813 * t464;
    let t2107 = t2106 * t477;
    let t2108 = t137 * t2107;
    let t2110 = t132 * t2108 / F::new(30.0);
    let t2111 = t1552 / F::new(45.0);
    let t2113 = t1637 / F::new(45.0);
    let t2114 = t2039 - t1550 - t2041 - t2045 - t2068 - t2070 - t2092 - t2097 - t2099 - t2103 - t2105 - t2110 + t2111 - t1557 - F::new(2.0) / F::new(45.0) * t1708 + t1712 - t2113 + t1732;
    (t2106, t2107, t2108, t2110, t2111, t2113, t2114)
}
