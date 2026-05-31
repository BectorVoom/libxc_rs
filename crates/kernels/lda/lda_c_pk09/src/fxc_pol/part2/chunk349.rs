//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 349/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk349<F: Float>(t1702: F, t1705: F, t1689: F, t1692: F, t1697: F, t253: F, t424: F, t1191: F, t418: F) -> (F, F, F, F, F) {
    let t1706 = t1702 * t1705;
    let t1709 = t1689 - t1692 + F::cast_from(1.28_f64) * t253 * t1697 - F::cast_from(1.28_f64) * t253 * t1706;
    let t1710 = t424 * t1709;
    let t1711 = t1710 * t1191;
    let t1713 = t253 * t418;
    (t1706, t1709, t1710, t1711, t1713)
}
