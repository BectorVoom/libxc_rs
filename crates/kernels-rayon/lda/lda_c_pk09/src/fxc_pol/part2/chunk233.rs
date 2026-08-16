//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 233/1113 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk233(t134: f64, t143: f64, t151: f64, t155: f64, t200: f64, t756: f64, t882: f64, t884: f64, t886: f64, t888: f64, t894: f64, t899: f64, t906: f64, t910: f64, t914: f64, t921: f64, t925: f64, t933: f64) -> f64 {
    let t938 = t882 + t884 - t886 + t888 - 1.800081713982063_f64 * t894 + 1.800081713982063_f64 * t899 + 1.800081713982063_f64 * t906 + 22.07984838129906_f64 * t910 + 22.07984838129906_f64 * t914 + t921 + t925 + 19.489173774580152_f64 * t155 * t756 + 1.8805371096875316_f64 * t151 * t756 - 3.7610742193750633_f64 * t143 * t756 - 0.6268457032291772_f64 * t933 * t134 - 2.427516195194328_f64 * t200 * t756;
    t938
}
