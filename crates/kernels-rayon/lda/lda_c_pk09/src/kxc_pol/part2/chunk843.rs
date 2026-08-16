//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 843/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk843(t61: f64, t82: f64, t971: f64, t8614: f64, t125: f64, t8374: f64, t204: f64, t1011: f64, t134: f64, t143: f64, t151: f64, t155: f64, t2183: f64, t4563: f64, t4566: f64, t8049: f64, t8600: f64, t8602: f64, t8604: f64, t8606: f64, t8608: f64, t8613: f64) -> (f64, f64, f64) {
    let t8615 = t61 * t82;
    let t8616 = t8615 * t971;
    let t8617 = t8614 * t8616;
    let t8620 = t8374 * t125;
    let t8621 = t8620 * t204;
    let t8631 = -22.07984838129906_f64 * t8600 - 44.15969676259812_f64 * t8602 - 22.07984838129906_f64 * t8604 - 22.07984838129906_f64 * t8606 - 44.15969676259812_f64 * t8608 - 2.427516195194328_f64 * t2183 * t1011 - 38.978347549160304_f64 * t8613 * t8617 - 0.6268457032291772_f64 * t8621 * t134 - 3.7610742193750633_f64 * t143 * t8049 + 1.8805371096875316_f64 * t151 * t8049 + 19.489173774580152_f64 * t155 * t8049 - 4.937333717448355_f64 * t4563 - t4566;
    (t8617, t8621, t8631)
}
