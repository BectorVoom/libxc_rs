//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 573/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk573<F: Float>(t1815: F, t2666: F, t639: F, t1675: F, t256: F, t2611: F, t2614: F, t2617: F, t2619: F, t2624: F, t2629: F, t2634: F, t2639: F, t2642: F, t2645: F, t2647: F, t2651: F, t2655: F, t2657: F, t2662: F, t2664: F, t2665: F) -> (F, F, F) {
    let t2667 = t1815 * t2666;
    let t2669 = 4.0 / 45.0 * t639 * t2667;
    let t2670 = -t2611 + t2614 + t2617 + t2619 + t2624 - t2629 + t2634 - t2639 + t2642 + t2645 + t2647 * t256 / 3.0 + t2651 / 3.0 + 0.60777777777777777777e-1 * t2655 + 2.0 / 9.0 * t2657 + t2662 + t2664 - t1675 + t2665 - t2669;
    (t2667, t2669, t2670)
}
