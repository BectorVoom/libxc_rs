//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 355/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk355(t1685: f64, t1732: f64, t1736: f64, t1738: f64, t1681: f64, t305: f64, t429: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1740 = t1685 - 1.5625_f64 * t1732 + t1736 + 1.5625_f64 * t1738;
    let t1741 = t1681 * t1740;
    let t1743 = 0.025613155472356368_f64 * t1681 + 1.0_f64;
    let t1744 = 1.0_f64 / t1743;
    let t1745 = t1744 * t305;
    let t1746 = t1741 * t1745;
    let t1747 = t429 * t68;
    (t1740, t1741, t1743, t1744, t1745, t1746, t1747)
}
