//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1016/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1016<F: Float>(t1290: F, t1294: F, t174: F, t331: F, t1305: F, t1314: F, t1319: F, t470: F, t1434: F, t4813: F, t18424: F, t18428: F, t18432: F, t18435: F, t18439: F, t18441: F, t18445: F, t18448: F, t18452: F) -> (F, F, F, F) {
    let t18456 = F::cast_from(0.2291123905095794067e1_f64) * t174 * t331 * t1290 * t1294;
    let t18460 = F::cast_from(0.21053604230838734656e2_f64) * t470 * t1319 * t1314 * t1305;
    let t18461 = t1434 * t4813;
    let t18462 = F::cast_from(0.22787712934626154593e-2_f64) * t18461;
    let t18463 = t18424 - t18428 + t18432 - t18435 + t18439 - t18441 - t18445 - t18448 - t18452 + t18456 - t18460 - t18462;
    (t18456, t18460, t18462, t18463)
}
