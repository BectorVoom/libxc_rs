//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 961/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk961<F: Float>(t11248: F, t1856: F, t11469: F, t1842: F, t1672: F, t2940: F, t11455: F, t11996: F, t12000: F, t12003: F, t12007: F, t12009: F, t12011: F, t12014: F, t12018: F, t12020: F, t12023: F, t1805: F, t1847: F, t2752: F, t455: F, t6907: F, t6911: F, t6924: F, t7489: F) -> (F,) {
    let t12026 = t1856 * t11248;
    let t12028 = t1842 * t11469;
    let t12030 = t2940 * t1672;
    let t12036 = 0.04115066352984959 * t11996 + 3.7610742193750633 * t7489 * t2752 - 19.489173774580152 * t12000 * t1805 + 19.489173774580152 * t12003 * t455 - 4.937333717448355 * t12007 - 4.738783832122567 * t12009 - 1.8805371096875316 * t12011 * t1805 + 1.8805371096875316 * t12014 * t455 - 1.8805371096875316 * t12018 - 4.937333717448355 * t12020 * t1805 + 4.937333717448355 * t12023 * t455 - 0.8091720650647759 * t12026 + 4.738783832122567 * t12028 + 0.8091720650647759 * t12030 + 7.108175748183851 * t1847 * t11455 + 1.1846959580306418 * t6907 - 4.738783832122567 * t6911 + t6924;
    (t12036,)
}
