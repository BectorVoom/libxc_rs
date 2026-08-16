//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 670/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk670(t3258: f64, t3757: f64, t2255: f64, t1133: f64, t274: f64, t343: f64, t1123: f64, t3123: f64, t3134: f64, t1220: f64, t1278: f64, t1288: f64, t1296: f64, t1328: f64, t1330: f64, t1335: f64, t1338: f64, t1440: f64, t1450: f64, t3341: f64, t3362: f64, t3702: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3758 = t3258 * t3757;
    let t3759 = t2255 * t3758;
    let t3762 = t274 * t1133;
    let t3763 = t3762 * t343;
    let t3764 = t1123 * t3763;
    let t3765 = t2255 * t3764;
    let t3769 = t3123 * t3134 / 48.0_f64;
    let t3770 = t1220 + t1328 - t1330 + t1335 + t1338 + t1450 - t1278 + t1288 + t1296 + t1440 - t3341 + t3362 - t3702;
    (t3759, t3763, t3764, t3765, t3769, t3770)
}
