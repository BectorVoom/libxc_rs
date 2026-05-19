//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 373/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk373<F: Float>(t1667: F, t1670: F, t1674: F, t1677: F, t1679: F, t1746: F, t1748: F, t1778: F, t1783: F, t1794: F, t1802: F, t1803: F, t1805: F, t1808: F, t1820: F, t1823: F, t455: F) -> F {
    let t1826 = t1667 + t1670 - t1674 + t1677 - t1679 - F::cast_from(2.9824072957409817_f64) * t1746 * t1748 - F::cast_from(2.427516195194328_f64) * t1778 * t455 - F::cast_from(1.7770439370459628_f64) * t1783 * t1794 + t1802 + F::cast_from(2.2140749178833072_f64) * t1803 * t1805 - F::cast_from(19.489173774580152_f64) * t1808 * t1805 + F::cast_from(19.489173774580152_f64) * t1820 * t455 + F::cast_from(3.7610742193750633_f64) * t1823 * t1805;
    t1826
}
