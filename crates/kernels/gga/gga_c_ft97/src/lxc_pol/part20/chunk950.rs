//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 950/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk950<F: Float>(t11593: F, t1901: F, t24882: F, t24884: F, t29052: F, t29057: F, t29060: F, t29064: F, t29068: F, t29073: F, t29077: F, t29084: F, t29087: F, t29090: F, t29094: F, t24890: F, t4146: F) -> (F, F) {
    let t29097 = -2.0 / 3.0 * t1901 * t29052 - 2.0 / 3.0 * t1901 * t29057 + t1901 * t29060 / 9.0 + t1901 * t29064 / 9.0 - 2.0 / 9.0 * t11593 * t29068 - 2.0 * t1901 * t29073 - 2.0 / 3.0 * t1901 * t29077 - t24882 / 9.0 - 2.0 / 9.0 * t24884 + t1901 * t29084 / 9.0 + t1901 * t29087 / 9.0 + 2.0 / 9.0 * t1901 * t29090 - 2.0 / 27.0 * t1901 * t29094;
    let t29098 = t24890 * t4146;
    (t29097, t29098)
}
