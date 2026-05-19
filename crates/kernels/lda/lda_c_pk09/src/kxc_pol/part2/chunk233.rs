//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 233/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk233<F: Float>(t134: F, t143: F, t151: F, t155: F, t200: F, t756: F, t882: F, t884: F, t886: F, t888: F, t894: F, t899: F, t906: F, t910: F, t914: F, t921: F, t925: F, t933: F) -> F {
    let t938 = t882 + t884 - t886 + t888 - F::cast_from(1.800081713982063_f64) * t894 + F::cast_from(1.800081713982063_f64) * t899 + F::cast_from(1.800081713982063_f64) * t906 + F::cast_from(22.07984838129906_f64) * t910 + F::cast_from(22.07984838129906_f64) * t914 + t921 + t925 + F::cast_from(19.489173774580152_f64) * t155 * t756 + F::cast_from(1.8805371096875316_f64) * t151 * t756 - F::cast_from(3.7610742193750633_f64) * t143 * t756 - F::cast_from(0.6268457032291772_f64) * t933 * t134 - F::cast_from(2.427516195194328_f64) * t200 * t756;
    t938
}
