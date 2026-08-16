//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1029/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1029<F: Float>(t11101: F, t477: F, t11092: F, t1828: F, t1832: F, t11059: F, t497: F, t507: F, t11080: F, t11084: F, t11087: F, t11093: F, t11096: F, t1746: F, t1748: F, t1808: F, t1820: F, t1823: F, t1859: F, t1873: F, t2032: F, t2740: F, t2783: F, t2872: F, t2877: F, t455: F, t6288: F) -> (F, F) {
    let t11102 = t11101 * t477;
    let t11107 = t1828 * t11092;
    let t11113 = t1832 * t11092;
    let t11115 = t497 * t11059;
    let t11118 = t507 * t11059;
    let t11121 = -F::cast_from(19.489173774580152_f64) * t1820 * t2783 - F::cast_from(2.9824072957409817_f64) * t2740 * t6288 - F::cast_from(2.9824072957409817_f64) * t11080 * t1748 - F::cast_from(19.489173774580152_f64) * t11084 - F::cast_from(1.8805371096875316_f64) * t11087 - F::cast_from(1.8805371096875316_f64) * t2872 * t2032 + F::cast_from(19.489173774580152_f64) * t11093 + F::cast_from(38.978347549160304_f64) * t1808 * t11096 - F::cast_from(7.5221484387501265_f64) * t1823 * t11096 - F::cast_from(2.9824072957409817_f64) * t1746 * t11102 - F::cast_from(19.489173774580152_f64) * t2877 * t2032 + F::cast_from(1.8805371096875316_f64) * t11107 + F::cast_from(3.7610742193750633_f64) * t1859 * t11096 - F::cast_from(1.8805371096875316_f64) * t1873 * t2783 - F::cast_from(3.7610742193750633_f64) * t11113 + F::cast_from(1.8805371096875316_f64) * t11115 * t455 + F::cast_from(4.937333717448355_f64) * t11118 * t455;
    (t11102, t11121)
}
