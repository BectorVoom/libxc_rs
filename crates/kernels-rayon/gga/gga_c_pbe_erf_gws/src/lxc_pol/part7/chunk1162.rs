//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1162/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1162(t20748: f64, t6672: f64, t6335: f64, t6342: f64, t6800: f64, t6605: f64, t6702: f64, t6183: f64, t6706: f64, t2120: f64, t20305: f64, t20720: f64, t20725: f64, t20731: f64, t20733: f64, t20734: f64, t20739: f64, t20746: f64, t2258: f64, t2345: f64, t3247: f64, t6275: f64, t6276: f64, t6287: f64, t904: f64) -> (f64, f64, f64, f64, f64) {
    let t20750 = t6672 * t20748 / 4.0_f64;
    let t20753 = t6800 * t6335 * t6342 / 16.0_f64;
    let t20754 = t6702 * t6605;
    let t20755 = 7.0_f64 / 36.0_f64 * t20754;
    let t20756 = t6183 * t6706;
    let t20757 = t2120 * t20756;
    let t20758 = 7.0_f64 / 72.0_f64 * t20757;
    let t20759 = -3.0_f64 / 32.0_f64 * t3247 * t2345 * t20305 * t6287 + 7.0_f64 / 576.0_f64 * t20720 - t20725 + t20731 - 5.0_f64 / 16.0_f64 * t20733 * t904 * t20734 * t2258 + t6275 * t6276 * t20739 / 16.0_f64 + t20746 + t20750 + t20753 + t20755 + t20758;
    (t20750, t20753, t20755, t20758, t20759)
}
