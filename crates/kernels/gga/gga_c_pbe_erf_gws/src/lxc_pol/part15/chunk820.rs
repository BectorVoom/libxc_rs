//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 820/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk820<F: Float>(t2637: F, t5312: F, t2825: F, t586: F, t593: F, t1037: F, t5470: F, t1627: F, t2593: F, t5478: F, t5482: F, t5437: F, t5443: F, t5449: F, t5452: F, t7775: F, t7779: F, t7780: F, t7781: F, t7784: F, t7788: F, t7790: F) -> (F, F, F, F, F, F, F) {
    let t7792 = 8.0 / 15.0 * t5312 * t2637;
    let t7793 = t2825 * t586;
    let t7795 = 8.0 / 45.0 * t7793 * t593;
    let t7797 = 4.0 / 45.0 * t5470 * t1037;
    let t7799 = 16.0 / 45.0 * t1627 * t2593;
    let t7800 = 8.0 / 135.0 * t5478;
    let t7801 = 8.0 / 81.0 * t5482;
    let t7802 = -4.0 / 27.0 * t5437 - t5443 + t5449 / 3.0 + 0.60777777777777777777e-1 * t5452 + t7775 + t7779 + t7780 - t7781 - t7784 - t7788 - t7790 - t7792 + t7795 + t7797 - t7799 + t7800 + t7801;
    (t7792, t7795, t7797, t7799, t7800, t7801, t7802)
}
