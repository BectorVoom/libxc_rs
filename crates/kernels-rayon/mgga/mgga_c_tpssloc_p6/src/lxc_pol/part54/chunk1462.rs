//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1462/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1462(t5: f64, t124814: f64, t124860: f64, t112: f64, t104977: f64, t117533: f64, t122718: f64, t122719: f64, t122720: f64, t122917: f64, t122920: f64, t124715: f64, t124728: f64, t1458: f64, t2039: f64, t24932: f64, t27170: f64, t27863: f64, t27888: f64, t32350: f64, t33152: f64, t33154: f64, t33690: f64, t4072: f64, t671: f64, t7056: f64, t7266: f64, t7801: f64, t8446: f64, t96238: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t124862 = piecewise3(t8, 0.0_f64, t124814 + t124860);
    let t124863 = t124862 * t112;
    let t124867 = 2.0_f64 * t104977 * t2039 + 2.0_f64 * t117533 * t1458 + 2.0_f64 * t122917 * t2039 + 2.0_f64 * t122920 * t2039 + 2.0_f64 * t124715 * t1458 + 2.0_f64 * t124728 * t671 + 2.0_f64 * t2039 * t96238 + 2.0_f64 * t24932 * t7801 + 2.0_f64 * t27170 * t7266 + 2.0_f64 * t27863 * t7056 + 2.0_f64 * t27888 * t7801 + 2.0_f64 * t32350 * t4072 + 2.0_f64 * t33690 * t7056 + 2.0_f64 * t122718 + 2.0_f64 * t122719 + 2.0_f64 * t122720 + t124863 + t33152 + t33154 + t8446;
    (t124863, t124867)
}
