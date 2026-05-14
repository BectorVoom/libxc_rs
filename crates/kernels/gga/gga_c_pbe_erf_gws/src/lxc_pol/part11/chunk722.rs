//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 722/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk722<F: Float>(t2612: F, t3407: F, t1022: F, t3473: F, t1809: F, t1620: F, t1044: F, t1815: F, t639: F, t12509: F, t12501: F, t2677: F, t3465: F, t3429: F, t995: F, t1821: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12611 = 8.0 / 15.0 * t2612 * t3407;
    let t12612 = t3473 * t1022;
    let t12613 = t1809 * t12612;
    let t12615 = 8.0 / 15.0 * t1620 * t12613;
    let t12616 = t3473 * t1044;
    let t12617 = t1815 * t12616;
    let t12619 = 4.0 / 15.0 * t639 * t12617;
    let t12620 = t1809 * t12509;
    let t12622 = 8.0 / 15.0 * t639 * t12620;
    let t12623 = t2677 * t12501;
    let t12625 = 4.0 / 9.0 * t639 * t12623;
    let t12626 = t3465 * t1022;
    let t12627 = t2677 * t12626;
    let t12629 = 8.0 / 9.0 * t1620 * t12627;
    let t12630 = t3429 * t995;
    let t12631 = t1821 * t12630;
    (t12611, t12612, t12613, t12615, t12616, t12617, t12619, t12620, t12622, t12623, t12625, t12626, t12627, t12629, t12630, t12631)
}
