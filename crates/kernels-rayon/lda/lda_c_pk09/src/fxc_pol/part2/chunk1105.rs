//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 1105/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk1105(t12203: f64, t12217: f64, t1871: f64, t452: f64, t11248: f64, t1803: f64, t11773: f64, t11776: f64, t12174: f64, t12175: f64, t12185: f64, t12187: f64, t455: f64, t7433: f64, t7467: f64, t7475: f64, t7484: f64, t7485: f64, t7488: f64, t7501: f64, t7504: f64, t7517: f64, t7522: f64, t7523: f64) -> f64 {
    let t12218 = t12203 + t12217;
    let t12219 = t12218 * t1871;
    let t12220 = t12219 * t452;
    let t12223 = t1803 * t11248;
    let t12225 = 2.2140749178833072_f64 * t7433 + 2.427516195194328_f64 * t7467 + 2.9824072957409817_f64 * t7475 - 38.978347549160304_f64 * t12174 * t12175 + 22.07984838129906_f64 * t11773 + 22.07984838129906_f64 * t11776 - t7484 + 1.8805371096875316_f64 * t7485 + t7488 + 3.7610742193750633_f64 * t7501 - 1.8805371096875316_f64 * t7504 - 19.489173774580152_f64 * t7517 + t7522 + 0.6268457032291772_f64 * t7523 + 0.7380249726277691_f64 * t12185 + 6.211752672544321_f64 * t12187 + 1.8805371096875316_f64 * t12220 * t455 - 0.7380249726277691_f64 * t12223;
    t12225
}
