//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 917/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk917<F: Float>(t11322: F, t1800: F, t11101: F, t489: F, t1971: F, t2871: F, t11299: F, t11302: F, t11304: F, t11306: F, t11312: F, t11314: F, t11317: F, t1805: F, t1844: F, t1849: F, t2744: F, t2752: F, t2832: F, t455: F, t6688: F, t6719: F, t6727: F, t6729: F, t6735: F, t6736: F, t7267: F) -> (F,) {
    let t11323 = t11322 * t1800;
    let t11325 = t489 * t11101;
    let t11330 = t2871 * t1971;
    let t11335 = -2.427516195194328 * t11299 * t455 - 1.2536914064583544 * t11302 - 6.496391258193384 * t11304 + 0.6268457032291772 * t11306 + 3.7610742193750633 * t2832 * t1849 + 3.7610742193750633 * t11312 - 3.7610742193750633 * t11314 * t1844 - 2.2140749178833072 * t11317 * t455 - 18.635258017632964 * t6688 * t2752 + 3.7610742193750633 * t11323 + 3.7610742193750633 * t11325 * t1805 - 1.8805371096875316 * t7267 * t2744 + 1.8805371096875316 * t11330 * t455 + 0.6268457032291772 * t6719 - t6727 - t6729 + t6735 - 0.8091720650647759 * t6736;
    (t11335,)
}
