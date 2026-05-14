//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 750/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk750<F: Float>(t204: F, t8620: F, t1011: F, t134: F, t143: F, t151: F, t155: F, t2183: F, t4563: F, t4566: F, t8049: F, t8600: F, t8602: F, t8604: F, t8606: F, t8608: F, t8613: F, t8617: F) -> (F, F) {
    let t8621 = t8620 * t204;
    let t8631 = -22.07984838129906 * t8600 - 44.15969676259812 * t8602 - 22.07984838129906 * t8604 - 22.07984838129906 * t8606 - 44.15969676259812 * t8608 - 2.427516195194328 * t2183 * t1011 - 38.978347549160304 * t8613 * t8617 - 0.6268457032291772 * t8621 * t134 - 3.7610742193750633 * t143 * t8049 + 1.8805371096875316 * t151 * t8049 + 19.489173774580152 * t155 * t8049 - 4.937333717448355 * t4563 - t4566;
    (t8621, t8631)
}
