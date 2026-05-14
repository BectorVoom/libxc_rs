//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 919/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk919<F: Float>(t574: F, t5842: F, t616: F, t1359: F, t2230: F, t2142: F, t5968: F, t144: F, t1384: F, t9428: F, t1378: F, t2178: F) -> (F, F, F, F, F, F, F) {
    let t23982 = t574 * t616 * t5842;
    let t23986 = t574 * t2230 * t1359;
    let t23989 = t2142 * t5968;
    let t23990 = t144 * t23989;
    let t23993 = t9428 * t1384;
    let t23994 = t144 * t23993;
    let t23997 = t1378 * t2178;
    (t23982, t23986, t23989, t23990, t23993, t23994, t23997)
}
