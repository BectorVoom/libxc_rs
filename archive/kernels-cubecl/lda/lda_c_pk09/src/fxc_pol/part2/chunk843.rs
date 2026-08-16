//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 843/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk843<F: Float>(t61: F, t82: F, t971: F, t8614: F, t125: F, t8374: F, t204: F, t1011: F, t134: F, t143: F, t151: F, t155: F, t2183: F, t4563: F, t4566: F, t8049: F, t8600: F, t8602: F, t8604: F, t8606: F, t8608: F, t8613: F) -> (F, F, F) {
    let t8615 = t61 * t82;
    let t8616 = t8615 * t971;
    let t8617 = t8614 * t8616;
    let t8620 = t8374 * t125;
    let t8621 = t8620 * t204;
    let t8631 = -F::cast_from(22.07984838129906_f64) * t8600 - F::cast_from(44.15969676259812_f64) * t8602 - F::cast_from(22.07984838129906_f64) * t8604 - F::cast_from(22.07984838129906_f64) * t8606 - F::cast_from(44.15969676259812_f64) * t8608 - F::cast_from(2.427516195194328_f64) * t2183 * t1011 - F::cast_from(38.978347549160304_f64) * t8613 * t8617 - F::cast_from(0.6268457032291772_f64) * t8621 * t134 - F::cast_from(3.7610742193750633_f64) * t143 * t8049 + F::cast_from(1.8805371096875316_f64) * t151 * t8049 + F::cast_from(19.489173774580152_f64) * t155 * t8049 - F::cast_from(4.937333717448355_f64) * t4563 - t4566;
    (t8617, t8621, t8631)
}
