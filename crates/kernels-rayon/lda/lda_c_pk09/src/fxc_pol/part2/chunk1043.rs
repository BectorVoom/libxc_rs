//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1043/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1043(t1971: f64, t2902: f64, t11101: f64, t1831: f64, t1800: f64, t489: f64, t2871: f64, t11299: f64, t11302: f64, t11304: f64, t11306: f64, t11312: f64, t11314: f64, t1805: f64, t1844: f64, t1849: f64, t2744: f64, t2752: f64, t2832: f64, t455: f64, t6688: f64, t6719: f64, t6727: f64, t6729: f64, t6735: f64, t6736: f64, t7267: f64) -> f64 {
    let t11317 = t2902 * t1971;
    let t11322 = t1831 * t11101;
    let t11323 = t11322 * t1800;
    let t11325 = t489 * t11101;
    let t11330 = t2871 * t1971;
    let t11335 = -2.427516195194328_f64 * t11299 * t455 - 1.2536914064583544_f64 * t11302 - 6.496391258193384_f64 * t11304 + 0.6268457032291772_f64 * t11306 + 3.7610742193750633_f64 * t2832 * t1849 + 3.7610742193750633_f64 * t11312 - 3.7610742193750633_f64 * t11314 * t1844 - 2.2140749178833072_f64 * t11317 * t455 - 18.635258017632964_f64 * t6688 * t2752 + 3.7610742193750633_f64 * t11323 + 3.7610742193750633_f64 * t11325 * t1805 - 1.8805371096875316_f64 * t7267 * t2744 + 1.8805371096875316_f64 * t11330 * t455 + 0.6268457032291772_f64 * t6719 - t6727 - t6729 + t6735 - 0.8091720650647759_f64 * t6736;
    t11335
}
