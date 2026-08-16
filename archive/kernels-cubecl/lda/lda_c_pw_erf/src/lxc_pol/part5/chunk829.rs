//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 829/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk829<F: Float>(t4049: F, t7612: F, t571: F, t2171: F, t2550: F, t2554: F, t523: F, t7360: F, t522: F, t519: F, t3894: F, t7354: F) -> (F, F, F, F, F, F, F, F) {
    let t7613 = t4049 * t7612;
    let t7615 = F::cast_from(32.0_f64) / F::cast_from(81.0_f64) * t571 * t7613;
    let t7617 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2171 * t2550;
    let t7619 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2171 * t2554;
    let t7620 = t523 * t7360;
    let t7621 = t522 * t7620;
    let t7623 = F::cast_from(4.0_f64) / F::cast_from(45.0_f64) * t519 * t7621;
    let t7624 = t3894 * t7354;
    (t7613, t7615, t7617, t7619, t7620, t7621, t7623, t7624)
}
