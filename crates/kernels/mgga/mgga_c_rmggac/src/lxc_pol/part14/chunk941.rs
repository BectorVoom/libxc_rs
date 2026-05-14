//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 941/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk941<F: Float>(t2139: F, t27: F, t3118: F, t558: F, t36634: F, t40972: F, t40975: F, t7192: F, t16156: F, t9194: F, t9190: F, t1001: F, t236: F, t3351: F, t35312: F, t551: F) -> (F, F, F, F, F, F) {
    let t42196 = t2139 * t27 * t3118 * t558;
    let t42199 = t36634 * t40972;
    let t42201 = t7192 * t40975;
    let t42204 = t16156 * t9194;
    let t42205 = 0.17877131955185092547e-3 * t42204;
    let t42206 = t16156 * t9190;
    let t42207 = 0.11918087970123395031e-3 * t42206;
    let t42211 = t3351 * t35312 * t236 * t551 * t1001;
    (t42196, t42199, t42201, t42205, t42207, t42211)
}
