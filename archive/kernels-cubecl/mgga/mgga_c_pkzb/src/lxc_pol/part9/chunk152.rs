//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 152/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk152<F: Float>(t430: F, t448: F, t453: F, t459: F, t52: F, t9: F) -> (F, F) {
    let t462 = F::cast_from(0.165625e-1_f64) * t430 * t448 - F::cast_from(0.165625e-1_f64) * t453 * t459;
    let t464 = t52 * t9;
    let t465 = F::cast_from(1.0_f64) / t464;
    (t462, t465)
}
