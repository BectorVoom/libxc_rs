//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 575/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk575<F: Float>(t640: F, t7556: F, t7555: F, t7553: F, t27: F, t3118: F, t684: F, t2124: F, t333: F, t884: F, t2123: F, t874: F) -> (F, F, F, F, F, F, F, F) {
    let t7557 = t640 * t7556;
    let t7558 = t7555 * t7557;
    let t7559 = t7553 * t7558;
    let t7561 = t27 * t3118;
    let t7562 = t684 * t7561;
    let t7564 = t2124 * t333;
    let t7565 = t884 * t7564;
    let t7566 = F::new(0.11974241701863808564e0) * t7565;
    let t7567 = t874 * t2123;
    (t7557, t7558, t7559, t7561, t7562, t7564, t7566, t7567)
}
