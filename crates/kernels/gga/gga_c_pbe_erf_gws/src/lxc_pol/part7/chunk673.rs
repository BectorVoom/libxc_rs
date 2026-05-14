//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 673/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk673<F: Float>(t1533: F, t507: F, t1553: F, t2718: F, t2704: F, t502: F, t1509: F, t486: F, t118: F, t119: F, t120: F, t837: F, t1552: F, t19: F, t506: F, t299: F, t481: F, t799: F) -> (F, F, F, F, F, F, F, F) {
    let t5744 = t507 * t1533;
    let t5749 = 0.65290666666666666667e0 * t1553 * t2718;
    let t5751 = 0.76172444444444444444e0 * t502 * t2704;
    let t5753 = 0.12991222222222222222e1 * t1509 * t2718;
    let t5755 = 0.15156425925925925926e1 * t486 * t2704;
    let t5759 = 7.0 / 27.0 * t118 * t119 * t837 * t120;
    let t5761 = t1552 * t506 * t19;
    let t5763 = t799 * t299 * t481;
    (t5744, t5749, t5751, t5753, t5755, t5759, t5761, t5763)
}
