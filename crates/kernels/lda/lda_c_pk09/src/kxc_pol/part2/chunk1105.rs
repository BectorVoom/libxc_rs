//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 1105/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk1105<F: Float>(t12203: F, t12217: F, t1871: F, t452: F, t11248: F, t1803: F, t11773: F, t11776: F, t12174: F, t12175: F, t12185: F, t12187: F, t455: F, t7433: F, t7467: F, t7475: F, t7484: F, t7485: F, t7488: F, t7501: F, t7504: F, t7517: F, t7522: F, t7523: F) -> F {
    let t12218 = t12203 + t12217;
    let t12219 = t12218 * t1871;
    let t12220 = t12219 * t452;
    let t12223 = t1803 * t11248;
    let t12225 = F::cast_from(2.2140749178833072_f64) * t7433 + F::cast_from(2.427516195194328_f64) * t7467 + F::cast_from(2.9824072957409817_f64) * t7475 - F::cast_from(38.978347549160304_f64) * t12174 * t12175 + F::cast_from(22.07984838129906_f64) * t11773 + F::cast_from(22.07984838129906_f64) * t11776 - t7484 + F::cast_from(1.8805371096875316_f64) * t7485 + t7488 + F::cast_from(3.7610742193750633_f64) * t7501 - F::cast_from(1.8805371096875316_f64) * t7504 - F::cast_from(19.489173774580152_f64) * t7517 + t7522 + F::cast_from(0.6268457032291772_f64) * t7523 + F::cast_from(0.7380249726277691_f64) * t12185 + F::cast_from(6.211752672544321_f64) * t12187 + F::cast_from(1.8805371096875316_f64) * t12220 * t455 - F::cast_from(0.7380249726277691_f64) * t12223;
    t12225
}
