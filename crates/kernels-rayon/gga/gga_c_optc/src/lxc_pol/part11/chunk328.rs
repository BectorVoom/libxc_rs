//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 328/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk328(t1422: f64, t1426: f64, t350: f64, t974: f64, t275: f64, t176: f64, t1000: f64, t1325: f64, t914: f64, t1331: f64, t1345: f64, t1371: f64, t1373: f64, t1377: f64, t1415: f64, t277: f64, t364: f64, t95: f64, t962: f64, t995: f64, t999: f64, sigma0: f64) -> (f64, f64, f64, f64, f64) {
    let t1428 = t1422 * t350 - t974 * t1426;
    let t1429 = t1428 * t275;
    let t1431 = t176 * t1429 * sigma0;
    let t1434 = t1000 * t1325;
    let t1435 = t914 * t1434;
    let t1438 = -t1331 + t1345 + t1371 + t1373 - t1377 + 0.25844881434903430496e-2_f64 * t95 * t277 * t1415 * t962 + t1431 * t364 / 2.0_f64 + t995 + t999 * t1435 / 6.0_f64;
    (t1428, t1431, t1434, t1435, t1438)
}
