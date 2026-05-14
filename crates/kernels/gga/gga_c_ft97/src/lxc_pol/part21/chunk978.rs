//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 978/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk978<F: Float>(t30357: F, t605: F, t144: F, t4458: F, t5855: F, t2221: F, t1060: F, t574: F, t6615: F, t1359: F, t4839: F, t30172: F, t3578: F, t6639: F, t1384: F, t4714: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30358 = t605 * t30357;
    let t30359 = t144 * t30358;
    let t30363 = t5855 * t4458;
    let t30364 = t2221 * t30363;
    let t30369 = t574 * t1060 * t6615;
    let t30373 = t574 * t4839 * t1359;
    let t30376 = t144 * t30172;
    let t30380 = t574 * t3578 * t6639;
    let t30383 = t1384 * t4714;
    (t30358, t30359, t30363, t30364, t30369, t30373, t30376, t30380, t30383)
}
