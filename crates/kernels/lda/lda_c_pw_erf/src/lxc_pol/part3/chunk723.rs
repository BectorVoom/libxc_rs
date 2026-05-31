//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 723/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk723<F: Float>(t1627: F, t1926: F, t20: F, t2259: F, t1639: F, t3707: F, t3736: F, t3749: F, t3760: F, t3764: F, t3785: F, t3789: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t4544 = t1926 * t1627;
    let t4546 = t2259 * t20;
    let t4547 = t4546 * t1639;
    let t4549 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t3707;
    let t4550 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t3736;
    let t4551 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t3749;
    let t4552 = F::cast_from(16.0_f64) / F::cast_from(135.0_f64) * t3760;
    let t4553 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t3764;
    let t4554 = F::cast_from(16.0_f64) / F::cast_from(405.0_f64) * t3785;
    let t4555 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t3789;
    (t4544, t4546, t4547, t4549, t4550, t4551, t4552, t4553, t4554, t4555)
}
