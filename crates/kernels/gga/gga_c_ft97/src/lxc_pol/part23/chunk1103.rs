//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1103/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1103<F: Float>(t1466: F, t2399: F, t6224: F, t25485: F, t6210: F, t6242: F, t6243: F, t96535: F, t25110: F, t444: F, t4113: F, t6249: F, t6250: F, t43585: F, t6: F, t8: F) -> (F, F, F, F, F, F, F) {
    let t98416 = t1466 * t2399 * t6224;
    let t98429 = t6210 * t25485;
    let t98432 = t6242 * t96535 * t6243;
    let t98519 = t25110 * t444;
    let t98520 = t4113 * t98519;
    let t98530 = t6249 * t96535 * t6250;
    let t98539 = t4113 * t43585 * t6 * t8;
    (t98416, t98429, t98432, t98519, t98520, t98530, t98539)
}
