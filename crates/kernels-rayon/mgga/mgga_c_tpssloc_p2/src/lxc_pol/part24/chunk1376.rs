//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1376/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1376(t225: f64, t23494: f64, t1011: f64, t3120: f64, t3040: f64, t6768: f64, t23384: f64, t23650: f64, t1023: f64, t1058: f64, t1060: f64, t10857: f64, t10913: f64, t11059: f64, t11060: f64, t1945: f64, t23327: f64, t23346: f64, t23601: f64, t23613: f64, t23621: f64, t23644: f64, t23692: f64, t23705: f64, t25429: f64, t25484: f64, t25491: f64, t25510: f64, t25511: f64, t25721: f64, t3180: f64, t3186: f64, t3188: f64, t3200: f64, t3201: f64, t4594: f64, t6680: f64, t6786: f64, t82730: f64) -> f64 {
    let t82750 = t23494 * t225;
    let t82754 = t3120 * t1011;
    let t82762 = t6768 * t3040;
    let t82789 = t23384 * t23650;
    let t82795 = -0.82246703342411321826e-2_f64 * t23327 * t82750 * t6786 - 0.24674011002723396548e-1_f64 * t23601 * t25491 * t82754 * t1023 + t1058 * t1945 * t10857 * t1060 + 6.0_f64 * t3186 * t82762 * t3188 - 3.0_f64 * t3200 * t82762 * t3201 + 3.0_f64 * t3180 * t23705 - 0.16449340668482264365e-1_f64 * t23327 * t25510 * t25511 * t10913 + 0.10966227112321509577e-1_f64 * t25429 * t25510 * t25721 * t10913 - 0.82246703342411321826e-2_f64 * t23327 * t23613 * t23692 + 6.0_f64 * t11059 * t82730 * t11060 + 0.49348022005446793095e-1_f64 * t23601 * t25484 * t82754 * t4594 - 0.82246703342411321826e-2_f64 * t82789 + 0.65797362673929057459e-1_f64 * t23346 * t23644 - 0.65797362673929057459e-1_f64 * t6680 * t23621;
    t82795
}
