//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1275/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1275<F: Float>(t1058: F, t5842: F, t1349: F, t26534: F, t376: F, t26788: F, t5766: F, t26539: F, t26545: F, t24087: F, t6580: F, t358: F, t11982: F, t1643: F, t1969: F, t2223: F, t24080: F, t24125: F, t26515: F, t26567: F, t27420: F, t27421: F, t28: F, t3588: F, t379: F, t5772: F, t5843: F, t9049: F, t94227: F) -> (F,) {
    let t104289 = t5842 * t1058;
    let t104306 = 2.0 / 9.0 * t1349 * t376 * t26534;
    let t104308 = 2.0 / 9.0 * t5766 * t26788;
    let t104311 = 2.0 / 9.0 * t1349 * t376 * t26539;
    let t104314 = 2.0 / 9.0 * t1349 * t376 * t26545;
    let t104316 = t6580 * t24087 / 9.0;
    let t104321 = t1058 * t358;
    let t104330 = -t5772 * t1969 * t104289 * t379 / 9.0 + t5766 * t26515 / 3.0 + t1349 * t28 * t24125 * t1058 / 6.0 - t94227 + t1349 * t28 * t5843 * t3588 / 3.0 + t104306 + t104308 + t104311 + t104314 - t104316 - t5772 * t9049 * t26567 * t1643 / 27.0 + 2.0 / 9.0 * t5772 * t24080 * t104321 * t2223 + t5772 * t27420 * t27421 * t11982 / 9.0;
    (t104330,)
}
