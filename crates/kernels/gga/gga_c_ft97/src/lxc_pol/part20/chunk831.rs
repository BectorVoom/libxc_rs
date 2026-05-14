//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 831/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk831<F: Float>(t25271: F, t2867: F, t15460: F, t2413: F, t6360: F, t2881: F, t2405: F, t4139: F, t6362: F, t8392: F, t1901: F, t25222: F, t25227: F, t25231: F, t25235: F, t25239: F, t25243: F, t25246: F, t25248: F, t25252: F, t25255: F, t25259: F, t25263: F, t25268: F, t446: F) -> (F, F, F, F, F, F, F, F) {
    let t25272 = t25271 * t2867;
    let t25273 = t15460 * t25272;
    let t25276 = t6360 * t2413;
    let t25277 = t2881 * t25276;
    let t25280 = t6360 * t2405;
    let t25281 = t4139 * t25280;
    let t25284 = t8392 * t6362;
    let t25286 = 2.0 / 3.0 * t446 * t25222 + t446 * t25227 / 3.0 + 2.0 / 3.0 * t446 * t25231 + 2.0 / 3.0 * t446 * t25235 + 4.0 / 3.0 * t446 * t25239 + 4.0 / 3.0 * t446 * t25243 - 4.0 / 9.0 * t25246 - 2.0 / 9.0 * t25248 + t25252 - 2.0 / 3.0 * t446 * t25255 - t446 * t25259 / 3.0 + 2.0 / 9.0 * t446 * t25263 - 2.0 / 3.0 * t446 * t25268 - 4.0 / 3.0 * t1901 * t25273 + t1901 * t25277 / 9.0 + 2.0 / 27.0 * t1901 * t25281 - 2.0 / 27.0 * t25284;
    (t25272, t25273, t25276, t25277, t25280, t25281, t25284, t25286)
}
