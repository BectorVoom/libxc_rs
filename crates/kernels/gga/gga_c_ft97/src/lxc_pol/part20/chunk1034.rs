//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1034/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1034<F: Float>(t1882: F, t24932: F, t25235: F, t25231: F, t25305: F, t1497: F, t89: F, t9555: F, t25135: F, t312: F, t25243: F, t25202: F, t2770: F, t6347: F, t24887: F, t8392: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t99107 = t1882 * t24932;
    let t99125 = t1882 * t25235;
    let t99127 = t1882 * t25231;
    let t99129 = t1882 * t25305;
    let t99140 = 28.0 / 81.0 * t89 * t9555 * t1497;
    let t99164 = t312 * t25135;
    let t99169 = t1882 * t25243;
    let t99180 = t1882 * t25202;
    let t99186 = t2770 * t6347;
    let t99197 = t8392 * t24887;
    (t99107, t99125, t99127, t99129, t99140, t99164, t99169, t99180, t99186, t99197)
}
