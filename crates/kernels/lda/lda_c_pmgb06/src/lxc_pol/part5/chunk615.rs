//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 615/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk615<F: Float>(t3955: F, t2164: F, t395: F, t1461: F, t842: F, t1447: F, t1995: F, t1435: F, t813: F, t1423: F, t1969: F, t810: F, t947: F) -> (F, F, F, F, F, F, F) {
    let t4571 = F::new(32.0) * t3955;
    let t4579 = F::cast_from(0.2133002709687175_f64) * t395 * t2164;
    let t4588 = t1461 * t842;
    let t4593 = F::new(4.0) / F::new(45.0) * t1447 * t1995;
    let t4619 = t1435 * t813;
    let t4624 = F::new(4.0) / F::new(45.0) * t1423 * t1969;
    let t4635 = t947 * t810;
    (t4571, t4579, t4588, t4593, t4619, t4624, t4635)
}
