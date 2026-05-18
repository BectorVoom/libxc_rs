//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 591/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk591<F: Float>(t2: F, t4516: F, t39: F, t784: F, t799: F, t1236: F, t119: F, t837: F, t391: F, t11: F, t1246: F, t398: F) -> (F, F, F, F, F, F, F, F) {
    let t4517 = t4516 * t2;
    let t4518 = t4517 * t39;
    let t4520 = t799 * t784;
    let t4521 = t1236 * t4520;
    let t4523 = t119 * t837;
    let t4524 = t391 * t4523;
    let t4527 = F::new(1.0)/pow_3_2::<f64>(t11);
    let t4528 = t4527 * t2;
    let t4529 = t4528 * t39;
    let t4531 = t1246 * t4520;
    let t4533 = t398 * t4523;
    (t4517, t4518, t4521, t4524, t4528, t4529, t4531, t4533)
}
