//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 373/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk373(t1667: f64, t1670: f64, t1674: f64, t1677: f64, t1679: f64, t1746: f64, t1748: f64, t1778: f64, t1783: f64, t1794: f64, t1802: f64, t1803: f64, t1805: f64, t1808: f64, t1820: f64, t1823: f64, t455: f64) -> f64 {
    let t1826 = t1667 + t1670 - t1674 + t1677 - t1679 - 2.9824072957409817_f64 * t1746 * t1748 - 2.427516195194328_f64 * t1778 * t455 - 1.7770439370459628_f64 * t1783 * t1794 + t1802 + 2.2140749178833072_f64 * t1803 * t1805 - 19.489173774580152_f64 * t1808 * t1805 + 19.489173774580152_f64 * t1820 * t455 + 3.7610742193750633_f64 * t1823 * t1805;
    t1826
}
