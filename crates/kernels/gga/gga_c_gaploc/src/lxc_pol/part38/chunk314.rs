//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 314/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk314<F: Float>(t779: F, t937: F, t2272: F, t286: F, t708: F, t1687: F, t2277: F, t1232: F, t1692: F, t1685: F, t2276: F, t716: F, t926: F) -> (F, F, F, F, F, F, F) {
    let t2509 = t779 * t937;
    let t2513 = t2272 * t286 * t708;
    let t2515 = t2277 * t1687;
    let t2517 = t1692 * t1232;
    let t2518 = t2276 * t1685;
    let t2519 = t2518 * M_PI;
    let t2520 = t2517 * t2519;
    let t2522 = t926 * t716;
    (t2509, t2513, t2515, t2518, t2519, t2520, t2522)
}
