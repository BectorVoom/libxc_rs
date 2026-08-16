//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 489/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk489(t1685: f64, t1736: f64, t2733: f64, t2736: f64, t1681: f64, t1745: f64, t132: f64, t2730: f64, t93: f64) -> (f64, f64, f64, f64, f64) {
    let t2738 = t1685 - 1.5625_f64 * t2733 + t1736 + 1.5625_f64 * t2736;
    let t2739 = t1681 * t2738;
    let t2740 = t2739 * t1745;
    let t2743 = t132 * t2730;
    let t2744 = t93 * t2743;
    (t2738, t2739, t2740, t2743, t2744)
}
