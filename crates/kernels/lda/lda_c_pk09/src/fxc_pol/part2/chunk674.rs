//! LDA_C_PK09 fxc pol — fxc_pol part 2 (v2rho2_1) CSE chunk 674/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_fxc_pol_part2_v2rho2_1_chunk674<F: Float>(t1927: F, t6292: F, t1468: F, t496: F, t1747: F, t4993: F, t95: F, t333: F) -> (F, F, F) {
    let t6294 = F::cast_from(18.635258017632964_f64) * t1927 * t6292;
    let t6299 = t496 * t1468;
    let t6300 = t6299 * t1747;
    let t6301 = t95 * t4993;
    let t6302 = t333 * t6301;
    (t6294, t6300, t6302)
}
