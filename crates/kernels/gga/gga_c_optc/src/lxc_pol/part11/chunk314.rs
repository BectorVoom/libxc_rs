//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 314/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk314<F: Float>(t1327: F, t1334: F, t1337: F, t1340: F, t812: F, t815: F) -> (F,) {
    let t1354 = 0.3529725e1 * t1334 - t812 - 0.516475e0 * t1327 + 0.6311625e0 * t1337 - t815 - 0.104195e0 * t1340;
    (t1354,)
}
