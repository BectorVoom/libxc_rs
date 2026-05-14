//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1223/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1223<F: Float>(t13580: F, t24389: F, t4939: F, t18123: F, t2917: F, t6045: F, t30674: F, t695: F, t224: F, t5009: F, t668: F, t2320: F, t505: F, t24330: F, t30785: F, t6055: F) -> (F, F, F, F, F, F) {
    let t123195 = t13580 * t24389 * t4939;
    let t123206 = t6045 * t2917 * t18123;
    let t123222 = t695 * t30674;
    let t123223 = t224 * t123222;
    let t123224 = t5009 * t668;
    let t123226 = t2320 * t123224 * t505;
    let t123230 = t24330 * t30785;
    let t123231 = t6055 * t123230;
    (t123195, t123206, t123223, t123226, t123230, t123231)
}
