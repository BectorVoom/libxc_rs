//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1037/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1037<F: Float>(t25135: F, t668: F, t1882: F, t25046: F, t1476: F, t9570: F, t2347: F, t6260: F, t25159: F, t25162: F, t24977: F, t2404: F, t2781: F, t25013: F, t6308: F, t681: F) -> (F, F, F, F, F, F, F, F) {
    let t99342 = t25135 * t668;
    let t99346 = t1882 * t25046;
    let t99352 = t1476 * t9570;
    let t99363 = t6260 * t2347;
    let t99368 = t25162 * t25159;
    let t99383 = t25162 * t24977;
    let t99391 = t2404 * t2781;
    let t99422 = t6308 * t681 * t25013;
    (t99342, t99346, t99352, t99363, t99368, t99383, t99391, t99422)
}
