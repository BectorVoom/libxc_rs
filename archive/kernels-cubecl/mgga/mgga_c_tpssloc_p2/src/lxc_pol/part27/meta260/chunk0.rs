//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1256/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1256<F: Float>(t533: F, t6995: F, t1390: F, t1983: F, t1388: F, t3701: F, t2019: F, t113: F, t1266: F, t1393: F, t1869: F, t1976: F, t1980: F, t510: F, t574: F, t650: F, t6515: F, t6517: F, t652: F, t6522: F, t6524: F, t6527: F, t6537: F, t6539: F, t672: F, t6862: F, t6872: F, t6877: F, t6882: F) -> (F, F, F, F, F) {
    let t6996 = t533 * t6995;
    let t6997 = t6996 * t1390;
    let t6998 = t1983 * t6997;
    let t6999 = t3701 * t1388;
    let t7000 = t2019 * t6999;
    let t7001 = t1983 * t7000;
    let t7002 = -t113 * t6862 - t1266 * t1869 + t1393 * t1980 - t1976 * t650 - t510 * t6515 + t574 * t6872 - F::cast_from(2.0_f64) * t6517 * t672 - F::cast_from(2.0_f64) * t652 * t6539 - t6522 - t6524 - t6527 - t6537 + t6877 + t6882 + t6998 - t7001;
    (t6996, t6997, t6999, t7000, t7002)
}
