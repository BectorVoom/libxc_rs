//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1102/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1102<F: Float>(t1466: F, t1478: F, t9555: F, t2404: F, t6222: F, t24989: F, t683: F, t2399: F, t6262: F, t6266: F, t1465: F, t1771: F, t6219: F, t6260: F, t7640: F, t458: F, t6209: F) -> (F, F, F, F, F, F, F, F, F) {
    let t98257 = 14.0 / 81.0 * t1466 * t9555 * t1478;
    let t98268 = t2404 * t6222;
    let t98273 = t683 * t24989;
    let t98306 = t1466 * t2399 * t6262;
    let t98309 = t1466 * t2399 * t6266;
    let t98317 = t1465 * t1771;
    let t98318 = t98317 * t6219;
    let t98359 = t7640 * t6260;
    let t98388 = t6209 * t458;
    (t98257, t98268, t98273, t98306, t98309, t98317, t98318, t98359, t98388)
}
