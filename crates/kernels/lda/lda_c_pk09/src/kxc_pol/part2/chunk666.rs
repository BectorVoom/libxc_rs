//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 666/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk666<F: Float>(t51: F, t151: F, t7693: F, t2146: F, t3012: F, t2: F, t629: F, t258: F, t630: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t7694 = t151 * t7693;
    let t7697 = t3012 * t2146;
    let t7700 = t629 * t2;
    let t7704 = piecewise3(t52, 0.0, -2.0 / 9.0 * t7697 * t630 - 2.0 / 3.0 * t7700 * t258);
    (t7694, t7704)
}
