//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 548/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk548<F: Float>(t1286: F, t1310: F, t1337: F, t5500: F, t5501: F, t6414: F, t6418: F, t6423: F, t6457: F, t6461: F, t6530: F, t6543: F, t6548: F, t6558: F, t6562: F, t6564: F, t88: F, t948: F) -> (F,) {
    let t6570 = t6414 * t1310 / 6.0 - t5500 - t5501 * t6418 / 18.0 - t1286 * t6423 / 3.0 + t1286 * t6457 / 6.0 + t1286 * t6461 / 6.0 - t948 * t1337 - t88 * t6562 + 2.0 * t6564 - 2.0 * t6530 - 2.0 * t6543 + 4.0 * t6548 - 2.0 * t6558;
    (t6570,)
}
