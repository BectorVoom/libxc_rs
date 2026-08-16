//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1029/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1029(t11101: f64, t477: f64, t11092: f64, t1828: f64, t1832: f64, t11059: f64, t497: f64, t507: f64, t11080: f64, t11084: f64, t11087: f64, t11093: f64, t11096: f64, t1746: f64, t1748: f64, t1808: f64, t1820: f64, t1823: f64, t1859: f64, t1873: f64, t2032: f64, t2740: f64, t2783: f64, t2872: f64, t2877: f64, t455: f64, t6288: f64) -> (f64, f64) {
    let t11102 = t11101 * t477;
    let t11107 = t1828 * t11092;
    let t11113 = t1832 * t11092;
    let t11115 = t497 * t11059;
    let t11118 = t507 * t11059;
    let t11121 = -19.489173774580152_f64 * t1820 * t2783 - 2.9824072957409817_f64 * t2740 * t6288 - 2.9824072957409817_f64 * t11080 * t1748 - 19.489173774580152_f64 * t11084 - 1.8805371096875316_f64 * t11087 - 1.8805371096875316_f64 * t2872 * t2032 + 19.489173774580152_f64 * t11093 + 38.978347549160304_f64 * t1808 * t11096 - 7.5221484387501265_f64 * t1823 * t11096 - 2.9824072957409817_f64 * t1746 * t11102 - 19.489173774580152_f64 * t2877 * t2032 + 1.8805371096875316_f64 * t11107 + 3.7610742193750633_f64 * t1859 * t11096 - 1.8805371096875316_f64 * t1873 * t2783 - 3.7610742193750633_f64 * t11113 + 1.8805371096875316_f64 * t11115 * t455 + 4.937333717448355_f64 * t11118 * t455;
    (t11102, t11121)
}
