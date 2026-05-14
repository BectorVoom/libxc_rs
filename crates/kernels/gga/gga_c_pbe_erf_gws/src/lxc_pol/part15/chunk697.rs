//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 697/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk697<F: Float>(t784: F, t799: F, t1236: F, t119: F, t837: F, t391: F, t11: F, t2: F, t39: F, t1246: F, t398: F, t2704: F, t2718: F, t4518: F, t404: F, t389: F) -> (F, F, F, F, F, F) {
    let t4520 = t799 * t784;
    let t4521 = t1236 * t4520;
    let t4523 = t119 * t837;
    let t4524 = t391 * t4523;
    let t4527 = 1.0/pow_3_2(t11);
    let t4528 = t4527 * t2;
    let t4529 = t4528 * t39;
    let t4531 = t1246 * t4520;
    let t4533 = t398 * t4523;
    let t4536 = -0.25319e1 * t4518 + 0.16879333333333333333e1 * t4521 - 0.19692555555555555555e1 * t4524 - 0.93011851851851851854e0 * t2704 + 0.13651666666666666667e0 * t4529 - 0.27303333333333333333e0 * t4531 - 0.3185388888888888889e0 * t4533 - 0.36514074074074074075e0 * t2718;
    let t4537 = t4536 * t404;
    let t4538 = t389 * t4537;
    (t4521, t4524, t4529, t4531, t4533, t4538)
}
