//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 996/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk996(t6969: f64, t6972: f64, t7176: f64, t9008: f64, t9029: f64, t9264: f64, t361: f64, t1422: f64, t1434: f64, t2533: f64, t2540: f64, t2555: f64, t2563: f64, t2572: f64, t2579: f64, t2602: f64, t3527: f64, t3547: f64, t3580: f64, t7099: f64, t7154: f64, t9205: f64, t9210: f64, t9242: f64, t9245: f64, t9248: f64, t9255: f64, t9260: f64, t979: f64, t988: f64) -> (f64, f64, f64) {
    let t9266 = -t7176 + 0.47488888888888888888e-1_f64 * t6969 - 0.17808333333333333333e-1_f64 * t6972 + 0.23744444444444444444e-1_f64 * t9008 - t9264 + 0.53425e-1_f64 * t9029;
    let t9268 = 0.621814e-1_f64 * t9266 * t361;
    let t9269 = 2.0_f64 * t9205 * t988 + 1.0_f64 * t3527 * t2555 + 0.32163958997385070134e2_f64 * t9210 * t2563 + 1.0_f64 * t7154 * t1422 + 2.0_f64 * t2533 * t3547 + 1.0_f64 * t979 * t9242 - 2.0_f64 * t9245 * t2540 + 0.17315859105681463759e2_f64 * t9248 * t2602 + 0.5848223622634646207e0_f64 * t7099 * t1434 + 0.11696447245269292414e1_f64 * t2572 * t3580 - 0.11696447245269292414e1_f64 * t9255 * t2579 + t9260 + t9268;
    (t9266, t9268, t9269)
}
