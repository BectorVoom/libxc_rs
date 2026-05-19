//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 900/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk900<F: Float>(t2818: F, t2820: F, t2916: F, t1138: F, t147: F, t8363: F, t2783: F, t688: F, t692: F, t285: F, t4130: F, t477: F) -> (F, F, F, F, F) {
    let t8812 = F::cast_from(1.8276876377896586e-05_f64) * t2818 * t2916 * t2820;
    let t8821 = F::cast_from(6.701521338562081e-05_f64) * t8363 * t147 * t1138 * t2820;
    let t8822 = t2783 * t688;
    let t8825 = F::cast_from(0.7805426614091894_f64) * t2783 * t692;
    let t8827 = t4130 * t477 * t285;
    (t8812, t8821, t8822, t8825, t8827)
}
