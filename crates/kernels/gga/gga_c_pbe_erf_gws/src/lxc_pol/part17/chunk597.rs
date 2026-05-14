//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 597/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk597<F: Float>(t1: F, t3: F, t991: F, t672: F, t2009: F, t2590: F, t2595: F, t2600: F, t2605: F, t2611: F, t2614: F, t2617: F, t2619: F, t2624: F, t2629: F, t2634: F, t2639: F, t2642: F, t2645: F, t2662: F, t2664: F) -> (F, F) {
    let t2970 = t991 * t1 * t3;
    let t2971 = t2970 * t672;
    let t2973 = t2009 + t2590 - t2595 - t2600 + t2605 - t2611 + t2614 + t2617 + t2619 + t2624 - t2629 + t2634 - t2639 + t2642 + t2645 + 0.10821041362364843377e0 * t2971 + t2662 + t2664;
    (t2970, t2973)
}
