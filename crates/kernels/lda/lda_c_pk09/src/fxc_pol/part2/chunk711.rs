//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 711/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk711<F: Float>(t495: F, t1745: F, t429: F, t1741: F, t6574: F, t6580: F, t6588: F, t6591: F, t463: F, t6601: F, t453: F, t6710: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6971 = t495 * t495;
    let t6972 = F::new(1.0) / t6971;
    let t6977 = t1745 * t429;
    let t6978 = t1741 * t6977;
    let t6981 = F::new(0.09983749558483038) * t6574;
    let t6982 = F::new(0.1110086767380779) * t6580;
    let t6984 = F::new(0.29951248675449116) * t6588;
    let t6985 = F::new(0.04933718966136796) * t6591;
    let t6989 = F::new(2.0) / F::new(27.0) * t463 * t6601;
    let t6991 = F::new(2.0) / F::new(27.0) * t453 * t6601;
    let t6995 = F::new(0.06655833038988691) * t6710;
    (t6972, t6977, t6978, t6981, t6982, t6984, t6985, t6989, t6991, t6995)
}
