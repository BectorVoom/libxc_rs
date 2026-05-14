//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1324/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1324<F: Float>(t1882: F, t27097: F, t2120: F, t28: F, t586: F, t5890: F, t6615: F, t1369: F, t27128: F, t376: F, t1637: F, t6677: F, t89: F, t23658: F, t925: F, t95292: F, t95293: F) -> (F, F, F, F, F, F, F, F) {
    let t105559 = t1882 * t27097;
    let t105560 = 4.0 / 9.0 * t105559;
    let t105564 = t5890 * t28 * t586 * t6615 * t2120;
    let t105567 = t1369 * t376 * t27128;
    let t105568 = 2.0 / 3.0 * t105567;
    let t105570 = t89 * t1637 * t6677;
    let t105571 = 8.0 / 9.0 * t105570;
    let t105574 = t95292 * t95293 * t925 * t23658;
    (t105559, t105560, t105564, t105567, t105568, t105570, t105571, t105574)
}
