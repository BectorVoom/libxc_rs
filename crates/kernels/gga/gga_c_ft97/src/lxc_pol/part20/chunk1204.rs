//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1204/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1204<F: Float>(t1466: F, t28862: F, t681: F, t1091: F, t1253: F, t1477: F, t14911: F, t1506: F, t15546: F, t193: F, t24989: F, t25459: F, t25465: F, t2665: F, t2682: F, t28978: F, t28987: F, t28997: F, t29000: F, t29035: F, t3746: F, t4027: F, t6210: F, t6216: F, t6391: F, t6970: F, t98407: F, t98429: F, t98653: F) -> (F,) {
    let t112549 = t1466 * t681 * t28862 / 9.0;
    let t112562 = t1466 * t193 * t24989 * t1253 * t2682 - t6216 * t2665 * t98653 * t1091 / 9.0 + 2.0 / 9.0 * t29000 * t2665 * t25465 * t3746 - t25459 * t28997 / 9.0 - t25459 * t28987 / 9.0 + t1466 * t193 * t1477 * t15546 / 6.0 + 4.0 / 27.0 * t98429 - t112549 - 2.0 * t4027 * t6391 - 2.0 * t14911 * t1506 - 2.0 / 3.0 * t6210 * t29035 - t1466 * t193 * t98407 * t6970 / 3.0 - 2.0 / 3.0 * t6210 * t28978;
    (t112562,)
}
