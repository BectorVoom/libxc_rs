//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 302/1113 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk302<F: Float>(t1240: F, t372: F, t310: F, t1337: F, t1284: F, t355: F) -> (F, F, F, F) {
    let t1338 = t372 * t1240;
    let t1339 = t310 * t1338;
    let t1341 = F::cast_from(0.04115066352984959_f64) * t1337 * t1339;
    let t1342 = t355 * t1284;
    (t1338, t1339, t1341, t1342)
}
