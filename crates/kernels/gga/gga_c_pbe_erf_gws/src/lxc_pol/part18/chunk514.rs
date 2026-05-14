//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 514/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk514<F: Float>(t1885: F, t2631: F, t587: F, t597: F, t995: F, t610: F, t1820: F, t1036: F, t1630: F, t639: F, t1009: F, t1651: F, t247: F, t2522: F, t251: F, t1061: F, t719: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t2632 = t1885 * t2631;
    let t2634 = 4.0 / 15.0 * t587 * t2632;
    let t2635 = t597 * t995;
    let t2636 = t2635 * t610;
    let t2637 = t1885 * t2636;
    let t2639 = 4.0 / 15.0 * t1820 * t2637;
    let t2640 = t1630 * t1036;
    let t2641 = t639 * t2640;
    let t2642 = 8.0 / 135.0 * t2641;
    let t2643 = t1651 * t1009;
    let t2644 = t587 * t2643;
    let t2645 = 8.0 / 135.0 * t2644;
    let t2646 = t2522 * t247;
    let t2647 = t2646 * t251;
    let t2650 = t1061 * t719;
    (t2632, t2634, t2635, t2636, t2637, t2639, t2640, t2641, t2642, t2643, t2644, t2645, t2646, t2647, t2650)
}
