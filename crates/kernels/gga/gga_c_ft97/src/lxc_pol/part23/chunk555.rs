//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 555/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk555<F: Float>(t317: F, t7022: F, t193: F, t1253: F, t1477: F, t1091: F, t6273: F, t2874: F, t1212: F, t1476: F) -> (F, F, F, F, F, F, F) {
    let t7023 = t7022 * t317;
    let t7024 = t193 * t7023;
    let t7027 = t1477 * t1253;
    let t7028 = t193 * t7027;
    let t7032 = t6273 * t1091;
    let t7033 = t2874 * t7032;
    let t7036 = t1476 * t1212;
    (t7023, t7024, t7027, t7028, t7032, t7033, t7036)
}
