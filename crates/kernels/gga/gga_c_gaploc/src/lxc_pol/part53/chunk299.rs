//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 299/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk299<F: Float>(t779: F, t937: F, t2272: F, t286: F, t708: F, t1687: F, t2277: F, t1232: F, t1692: F, t1685: F, t2276: F, t716: F, t926: F, t471: F, t64: F, t931: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2509 = t779 * t937;
    let t2513 = t2272 * t286 * t708;
    let t2515 = t2277 * t1687;
    let t2517 = t1692 * t1232;
    let t2518 = t2276 * t1685;
    let t2519 = t2518 * M_PI;
    let t2520 = t2517 * t2519;
    let t2522 = t926 * t716;
    let t2524 = -21.0 / 256.0 * t2513 + 21.0 / 8192.0 * t2515 - 7.0 / 8192.0 * t2520 + 7.0 / 256.0 * t2522;
    let t2530 = t2524 * t471 - 4.0 / 3.0 * t931 * t64 - 7.0 / 256.0 * t2513 + 7.0 / 768.0 * t2522;
    (t2509, t2513, t2515, t2518, t2519, t2520, t2522, t2524, t2530)
}
