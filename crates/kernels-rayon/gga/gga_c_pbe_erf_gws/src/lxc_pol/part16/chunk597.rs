//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 597/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk597(t2705: f64, t657: f64, t1688: f64, t1689: f64, t1709: f64, t1710: f64, t25: f64, t2696: f64, t2699: f64, t2702: f64, t2707: f64, t2710: f64, t2712: f64, t2715: f64, t2718: f64) -> (f64, f64) {
    let t2719 = t657 * t2705;
    let t2722 = t1688 + 0.11997222222222222222e-1_f64 * t1689 + 0.11997222222222222222e-1_f64 * t2696 - 0.23994444444444444445e-1_f64 * t2699 + 0.71983333333333333334e-1_f64 * t2702 + 0.71983333333333333334e-1_f64 * t2707 + t1709 + 0.44444444444444444445e-2_f64 * t1710 + 0.44444444444444444445e-2_f64 * t2710 - 0.22222222222222222222e-2_f64 * t25 * t2712 + 0.13333333333333333333e-1_f64 * t25 * t2715 + 0.13333333333333333333e-1_f64 * t2718 * t2719;
    (t2719, t2722)
}
