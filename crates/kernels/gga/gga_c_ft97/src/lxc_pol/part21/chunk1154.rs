//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1154/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1154<F: Float>(t116285: F, t446: F, t7824: F, t116320: F, t102109: F, t102110: F, t116383: F, t116387: F, t116390: F, t116393: F, t116396: F, t116400: F, t116402: F, t92202: F, t116324: F, t1317: F, t29722: F, t376: F) -> (F, F, F, F, F) {
    let t116405 = t446 * t7824 * t116285;
    let t116408 = t446 * t7824 * t116320;
    let t116410 = 3.0 / 2.0 * t116383 + 12.0 * t116387 + 8.0 / 3.0 * t116390 - 8.0 / 9.0 * t116393 + t116396 + t92202 - t116400 + t102109 + t102110 - 8.0 / 3.0 * t116402 + 4.0 / 3.0 * t116405 - 4.0 / 3.0 * t116408;
    let t116414 = t446 * t7824 * t116324;
    let t116417 = t1317 * t376 * t29722;
    (t116405, t116408, t116410, t116414, t116417)
}
