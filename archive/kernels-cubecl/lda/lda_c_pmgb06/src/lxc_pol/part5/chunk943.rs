//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 943/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk943<F: Float>(t13708: F, t1710: F, t801: F, t446: F, t3259: F, t813: F, t1969: F, t3213: F, t1886: F, t607: F, t1966: F, t3031: F) -> (F, F, F, F, F, F, F) {
    let t13709 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t13708;
    let t13712 = t801 * t1710;
    let t13713 = t13712 * t446;
    let t13714 = F::cast_from(2.0_f64) / F::cast_from(135.0_f64) * t13713;
    let t13715 = t3259 * t813;
    let t13719 = t3213 * t1969;
    let t13720 = F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t13719;
    let t13726 = t1886 * t607;
    let t13788 = t1966 * t3031;
    (t13709, t13712, t13714, t13715, t13720, t13726, t13788)
}
