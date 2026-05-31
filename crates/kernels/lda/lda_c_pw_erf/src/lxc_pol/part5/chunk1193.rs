//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1193/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1193<F: Float>(t668: F, t8025: F, t17458: F, t17461: F, t571: F, t7723: F, t9278: F, t108: F, t15060: F, t19249: F, t21605: F, t21608: F, t21611: F, t21614: F, t21617: F, t21622: F, t21624: F, t21657: F, t267: F) -> (F, F, F, F) {
    let t21661 = t8025 * t668;
    let t21664 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t17458;
    let t21665 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t17461;
    let t21667 = t571 * t9278 * t7723;
    let t21668 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t21667;
    let t21669 = t21605 + t21608 - t21611 - t21614 + t21617 - t21622 + F::cast_from(0.09973633333333333_f64) * t19249 + t21624 - t21657 * t108 * t267 / F::cast_from(15.0_f64) - F::cast_from(2.0_f64) / F::cast_from(45.0_f64) * t21661 - F::cast_from(0.040518518518518516_f64) * t15060 + t21664 - t21665 - t21668;
    (t21664, t21665, t21668, t21669)
}
