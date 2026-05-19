//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1043/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1043<F: Float>(t1971: F, t2902: F, t11101: F, t1831: F, t1800: F, t489: F, t2871: F, t11299: F, t11302: F, t11304: F, t11306: F, t11312: F, t11314: F, t1805: F, t1844: F, t1849: F, t2744: F, t2752: F, t2832: F, t455: F, t6688: F, t6719: F, t6727: F, t6729: F, t6735: F, t6736: F, t7267: F) -> F {
    let t11317 = t2902 * t1971;
    let t11322 = t1831 * t11101;
    let t11323 = t11322 * t1800;
    let t11325 = t489 * t11101;
    let t11330 = t2871 * t1971;
    let t11335 = -F::cast_from(2.427516195194328_f64) * t11299 * t455 - F::cast_from(1.2536914064583544_f64) * t11302 - F::cast_from(6.496391258193384_f64) * t11304 + F::cast_from(0.6268457032291772_f64) * t11306 + F::cast_from(3.7610742193750633_f64) * t2832 * t1849 + F::cast_from(3.7610742193750633_f64) * t11312 - F::cast_from(3.7610742193750633_f64) * t11314 * t1844 - F::cast_from(2.2140749178833072_f64) * t11317 * t455 - F::cast_from(18.635258017632964_f64) * t6688 * t2752 + F::cast_from(3.7610742193750633_f64) * t11323 + F::cast_from(3.7610742193750633_f64) * t11325 * t1805 - F::cast_from(1.8805371096875316_f64) * t7267 * t2744 + F::cast_from(1.8805371096875316_f64) * t11330 * t455 + F::cast_from(0.6268457032291772_f64) * t6719 - t6727 - t6729 + t6735 - F::cast_from(0.8091720650647759_f64) * t6736;
    t11335
}
