//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1153/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1153<F: Float>(t1466: F, t2399: F, t6971: F, t25462: F, t28997: F, t11176: F, t1465: F, t29002: F, t6967: F, t98317: F, t1253: F, t6260: F, t1476: F, t4309: F, t1464: F, t1900: F, t7149: F) -> (F, F, F, F, F, F, F) {
    let t111682 = t1466 * t2399 * t6971;
    let t111685 = t25462 * t28997 / 27.0;
    let t111687 = t1465 * t11176 * t29002;
    let t111705 = t98317 * t6967;
    let t111711 = t6260 * t1253;
    let t111716 = t1476 * t4309;
    let t111732 = t1464 * t7149 * t1900;
    (t111682, t111685, t111687, t111705, t111711, t111716, t111732)
}
