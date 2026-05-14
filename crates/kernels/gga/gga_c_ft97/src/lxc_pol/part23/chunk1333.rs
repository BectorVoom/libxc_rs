//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1333/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1333<F: Float>(t31891: F, t8392: F, t31932: F, t681: F, t89: F, t10703: F, t113972: F, t113974: F, t113980: F, t113981: F, t114001: F, t114222: F, t125572: F, t15369: F, t15460: F, t1901: F, t19339: F, t19404: F, t19423: F, t24890: F, t25271: F, t2862: F, t29129: F, t296: F, t31613: F, t446: F, t5408: F, t882: F, t99030: F, t99032: F) -> (F,) {
    let t126570 = t8392 * t31891;
    let t126587 = t89 * t681 * t31932;
    let t126593 = 4.0 / 3.0 * t1901 * t15369 * t25271 * t19404 + t1901 * t24890 * t19339 / 9.0 - 2.0 / 81.0 * t126570 + 4.0 / 3.0 * t446 * t2862 * t882 * t31613 + 8.0 / 27.0 * t99030 + 4.0 / 27.0 * t99032 - t113972 + t113974 + t113980 - 8.0 / 27.0 * t113981 + 2.0 * t1901 * t15460 * t29129 * t19423 + 2.0 / 3.0 * t446 * t296 * t125572 + t114001 - t126587 / 9.0 - 2.0 / 9.0 * t1901 * t10703 * t114222 * t5408;
    (t126593,)
}
