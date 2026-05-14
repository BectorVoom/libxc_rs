//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 637/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk637<F: Float>(t1882: F, t2182: F, t2187: F, t2202: F, t161: F, t7943: F, t89: F, t1901: F, t446: F, t9402: F, t9405: F, t9408: F, t9412: F, t9416: F, t9420: F, t9425: F, t9430: F, t9434: F, t9442: F, t9446: F) -> (F,) {
    let t9449 = t1882 * t2182;
    let t9451 = t1882 * t2187;
    let t9453 = t1882 * t2202;
    let t9457 = 28.0 / 81.0 * t89 * t7943 * t161;
    let t9458 = 2.0 * t446 * t9402 + 2.0 / 9.0 * t9405 - t446 * t9408 / 3.0 - 2.0 / 3.0 * t446 * t9412 + 4.0 / 9.0 * t446 * t9416 + 2.0 / 3.0 * t1901 * t9420 + t1901 * t9425 / 3.0 - t446 * t9430 - 2.0 * t446 * t9434 - 2.0 * t446 * t9442 + 2.0 * t446 * t9446 - 2.0 / 3.0 * t9449 - 2.0 / 3.0 * t9451 + t9453 / 9.0 - t9457;
    (t9458,)
}
