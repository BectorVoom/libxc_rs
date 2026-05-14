//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1108/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1108<F: Float>(t20748: F, t20751: F, t17351: F, t17354: F, t17357: F, t17405: F, t17408: F, t17411: F, t17414: F, t17417: F, t17487: F, t17505: F, t20705: F, t20708: F, t20710: F, t20719: F, t20745: F, t20754: F, t21055: F) -> (F,) {
    let t21058 = 0.104195e1 * t20748;
    let t21059 = 0.104195e1 * t20751;
    let t21062 = -0.27785333333333333333e1 * t17405 + 0.104195e1 * t17411 - 0.62517e0 * t17414 - 0.20839e0 * t17417 - 0.16068111111111111111e1 * t20705 + 0.794188125e1 * t20708 - 0.473371875e0 * t20710 + t17505 - 0.48204333333333333334e1 * t17351 + 0.20659e1 * t17354 - 0.516475e0 * t17357 + t21055 - 0.1549425e1 * t20719 + 0.1549425e1 * t20745 + t21058 + t21059 - 0.92617777777777777779e0 * t20754 + t17487 + 0.104195e1 * t17408;
    (t21062,)
}
