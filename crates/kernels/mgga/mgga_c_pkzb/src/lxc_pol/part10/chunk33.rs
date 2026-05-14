//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 33/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk33<F: Float>(t65: F, t68: F, t55: F, t58: F, t61: F) -> (F, F, F, F) {
    let t69 = t65 * t68;
    let t71 = 0.379785e1 * t58 + 0.8969e0 * t55 + 0.204775e0 * t61 + 0.123235e0 * t69;
    let t74 = 1.0 + 0.16081979498692535067e2 / t71;
    let t75 = f64::ln(t74);
    (t69, t71, t74, t75)
}
