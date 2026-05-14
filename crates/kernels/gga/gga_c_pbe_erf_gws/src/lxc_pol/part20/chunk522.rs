//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 522/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk522<F: Float>(t2705: F, t657: F, t1688: F, t1689: F, t1709: F, t1710: F, t25: F, t2696: F, t2699: F, t2702: F, t2707: F, t2710: F, t2712: F, t2715: F, t2718: F, t650: F) -> (F, F, F) {
    let t2719 = t657 * t2705;
    let t2722 = t1688 + 0.11997222222222222222e-1 * t1689 + 0.11997222222222222222e-1 * t2696 - 0.23994444444444444445e-1 * t2699 + 0.71983333333333333334e-1 * t2702 + 0.71983333333333333334e-1 * t2707 + t1709 + 0.44444444444444444445e-2 * t1710 + 0.44444444444444444445e-2 * t2710 - 0.22222222222222222222e-2 * t25 * t2712 + 0.13333333333333333333e-1 * t25 * t2715 + 0.13333333333333333333e-1 * t2718 * t2719;
    let t2723 = t650 * t2722;
    (t2719, t2722, t2723)
}
