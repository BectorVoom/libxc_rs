//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1210/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1210<F: Float>(t20716: F, t20748: F, t20751: F, t17351: F, t17354: F, t17357: F, t17405: F, t17408: F, t17411: F, t17414: F, t17417: F, t17487: F, t17505: F, t20705: F, t20708: F, t20710: F, t20719: F, t20745: F, t20754: F) -> F {
    let t21055 = F::cast_from(0.20659e1_f64) * t20716;
    let t21058 = F::cast_from(0.104195e1_f64) * t20748;
    let t21059 = F::cast_from(0.104195e1_f64) * t20751;
    let t21062 = -F::cast_from(0.27785333333333333333e1_f64) * t17405 + F::cast_from(0.104195e1_f64) * t17411 - F::cast_from(0.62517e0_f64) * t17414 - F::cast_from(0.20839e0_f64) * t17417 - F::cast_from(0.16068111111111111111e1_f64) * t20705 + F::cast_from(0.794188125e1_f64) * t20708 - F::cast_from(0.473371875e0_f64) * t20710 + t17505 - F::cast_from(0.48204333333333333334e1_f64) * t17351 + F::cast_from(0.20659e1_f64) * t17354 - F::cast_from(0.516475e0_f64) * t17357 + t21055 - F::cast_from(0.1549425e1_f64) * t20719 + F::cast_from(0.1549425e1_f64) * t20745 + t21058 + t21059 - F::cast_from(0.92617777777777777779e0_f64) * t20754 + t17487 + F::cast_from(0.104195e1_f64) * t17408;
    t21062
}
