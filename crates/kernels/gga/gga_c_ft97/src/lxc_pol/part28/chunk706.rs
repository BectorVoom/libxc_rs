//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 706/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk706<F: Float>(t32493: F, t32539: F, t32590: F, t32639: F, t1286: F, t32392: F, t32396: F, t32401: F, t32403: F, t32406: F, t32412: F, t32415: F, t32417: F, t32420: F, t32425: F, t32428: F, t32458: F, t32470: F, t32474: F, t32546: F, t32550: F, t438: F, t5501: F, t5510: F, t7162: F, t7286: F, t88: F) -> (F, F) {
    let t32641 = t32493 + t32539 + t32590 + t32639;
    let t32649 = -t1286 * t32392 / 3.0 + t1286 * t32396 / 3.0 + t32401 + t1286 * t32403 - 2.0 / 3.0 * t1286 * t32406 - t7162 * t5510 / 3.0 + 8.0 * t32412 + 4.0 * t32415 + 8.0 * t32417 - 12.0 * t32420 - t438 * t7286 - t5501 * t32425 / 9.0 - t88 * t32641 - 2.0 * t32458 - 2.0 * t32470 + 4.0 * t32474 - 4.0 * t32428 - 2.0 * t32546 - 4.0 * t32550;
    (t32641, t32649)
}
