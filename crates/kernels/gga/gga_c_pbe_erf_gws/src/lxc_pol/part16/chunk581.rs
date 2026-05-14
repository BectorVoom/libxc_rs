//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 581/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk581<F: Float>(t2750: F, t1780: F, t2676: F, t2682: F, t2687: F, t2691: F, t2692: F, t2693: F, t2694: F, t2726: F, t2728: F, t2732: F, t2734: F, t2739: F, t2743: F, t2745: F, t2746: F, t2748: F) -> (F, F) {
    let t2751 = 4.0 / 45.0 * t2750;
    let t2752 = -t2676 + t2682 - t2687 - t2691 - t2692 - t2693 - t1780 + t2694 - t2726 - t2728 + t2732 + t2734 - t2739 + t2743 - t2745 + t2746 + t2748 - t2751;
    (t2751, t2752)
}
