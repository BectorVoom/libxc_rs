//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 851/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk851<F: Float>(t10488: F, t7194: F, t1620: F, t2570: F, t34: F, t2612: F, t2685: F, t2572: F, t7527: F, t3562: F, t626: F, t422: F, t1809: F, t3553: F, t1815: F, t639: F) -> (F, F, F, F, F, F) {
    let t10489 = t7194 * t10488;
    let t10491 = 16.0 / 45.0 * t1620 * t10489;
    let t10492 = t2570 * t34;
    let t10493 = t7194 * t10492;
    let t10495 = 32.0 / 45.0 * t1620 * t10493;
    let t10497 = 8.0 / 45.0 * t2612 * t2685;
    let t10499 = 16.0 / 45.0 * t7527 * t2572;
    let t10500 = t3562 * t626;
    let t10501 = t10500 * t422;
    let t10502 = t1809 * t10501;
    let t10504 = 8.0 / 45.0 * t1620 * t10502;
    let t10505 = t3553 * t626;
    let t10506 = t10505 * t422;
    let t10507 = t1815 * t10506;
    let t10509 = 4.0 / 45.0 * t639 * t10507;
    (t10491, t10495, t10497, t10499, t10504, t10509)
}
