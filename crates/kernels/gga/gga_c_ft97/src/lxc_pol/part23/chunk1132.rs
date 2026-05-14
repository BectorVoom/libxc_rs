//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1132/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1132<F: Float>(t209: F, t8: F, t420: F, t24269: F, t9533: F, t96599: F, t24373: F, t109063: F, t3771: F, t2393: F, t6027: F, t6032: F, t65750: F, t2378: F, t24378: F, t27633: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109246 = t8 * t209;
    let t109247 = t109246 * t420;
    let t109266 = t420 * t24269;
    let t109272 = t9533 * t96599;
    let t109273 = t420 * t24373;
    let t109303 = t3771 * t109063;
    let t109304 = t6027 * t2393;
    let t109309 = t3771 * t6032 * t65750;
    let t109310 = t6027 * t2378;
    let t109314 = t24378 * t27633;
    (t109247, t109266, t109272, t109273, t109303, t109304, t109309, t109310, t109314)
}
