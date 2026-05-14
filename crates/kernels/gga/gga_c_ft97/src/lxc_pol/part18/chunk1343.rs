//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1343/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1343<F: Float>(t105856: F, t1969: F, t446: F, t920: F, t95234: F, t1882: F, t27040: F, t1986: F, t40830: F, t6630: F, t27157: F, t3526: F, t574: F, t590: F, t5900: F, t1039: F, t2120: F) -> (F, F, F, F, F, F, F) {
    let t105857 = 2.0 / 9.0 * t105856;
    let t105860 = t446 * t1969 * t95234 * t920;
    let t105862 = t1882 * t27040;
    let t105863 = 4.0 * t105862;
    let t105866 = t446 * t40830 * t6630 * t1986;
    let t105871 = t27157 * t574 * t5900 * t3526 * t590;
    let t105876 = t27157 * t574 * t5900 * t1039 * t2120;
    (t105857, t105860, t105862, t105863, t105866, t105871, t105876)
}
