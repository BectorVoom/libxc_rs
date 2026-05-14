//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 677/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk677<F: Float>(t14690: F, t2665: F, t446: F, t2680: F, t4129: F, t824: F, t193: F, t89: F, t2739: F, t4056: F, t1212: F, t2682: F, t7640: F, t10400: F, t10279: F, t1186: F, t9733: F) -> (F, F, F, F, F, F, F) {
    let t14691 = t2665 * t14690;
    let t14692 = t446 * t14691;
    let t14694 = t2680 * t4129;
    let t14695 = t14694 * t824;
    let t14697 = t89 * t193 * t14695;
    let t14699 = t4056 * t2739;
    let t14701 = t89 * t193 * t14699;
    let t14704 = t7640 * t1212 * t2682;
    let t14706 = t89 * t193 * t14704;
    let t14708 = 4.0 / 27.0 * t10400;
    let t14711 = 4.0 / 81.0 * t10279;
    let t14715 = t89 * t9733 * t1186;
    (t14692, t14697, t14701, t14706, t14708, t14711, t14715)
}
