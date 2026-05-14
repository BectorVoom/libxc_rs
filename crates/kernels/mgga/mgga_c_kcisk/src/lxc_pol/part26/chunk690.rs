//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 690/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk690<F: Float>(t1341: F, t8072: F, t3785: F, t1411: F, t2152: F, t2231: F) -> (F, F, F, F) {
    let t8073 = t1341 * t8072;
    let t8074 = t3785 * t8073;
    let t8075 = t1411 * t8074;
    let t8077 = t2231 * t2152;
    (t8073, t8074, t8075, t8077)
}
