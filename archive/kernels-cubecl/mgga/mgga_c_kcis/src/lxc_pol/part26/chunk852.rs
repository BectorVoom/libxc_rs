//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 852/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk852<F: Float>(t12321: F, t41: F, t4291: F, t5747: F, t2033: F, t4121: F, t492: F, t6015: F, t1466: F, t5997: F, t11825: F, t12534: F, t251: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t17382 = t41 * t12321;
    let t17391 = t5747 * t4291;
    let t17396 = t2033 * t4121;
    let t17412 = t6015 * t492;
    let t17449 = t5997 * t1466;
    let t17450 = t17449 * sigma2;
    let t17463 = t11825 * t4291;
    let t17470 = t251 * t12534;
    (t17382, t17391, t17396, t17412, t17449, t17450, t17463, t17470)
}
