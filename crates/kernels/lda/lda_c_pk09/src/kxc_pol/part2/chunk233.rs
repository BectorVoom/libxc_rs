//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 233/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk233<F: Float>(t134: F, t143: F, t151: F, t155: F, t200: F, t756: F, t882: F, t884: F, t886: F, t888: F, t894: F, t899: F, t906: F, t910: F, t914: F, t921: F, t925: F, t933: F) -> F {
    let t938 = t882 + t884 - t886 + t888 - F::new(1.800081713982063) * t894 + F::new(1.800081713982063) * t899 + F::new(1.800081713982063) * t906 + F::new(22.07984838129906) * t910 + F::new(22.07984838129906) * t914 + t921 + t925 + F::new(19.489173774580152) * t155 * t756 + F::new(1.8805371096875316) * t151 * t756 - F::new(3.7610742193750633) * t143 * t756 - F::new(0.6268457032291772) * t933 * t134 - F::new(2.427516195194328) * t200 * t756;
    t938
}
