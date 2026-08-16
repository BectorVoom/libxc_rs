//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 964/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk964(t8840: f64, t8852: f64, t810: f64, t788: f64, t3363: f64, t820: f64, t1351: f64, t2272: f64, t3313: f64, t787: f64, t811: f64, t2253: f64, t2268: f64, t2273: f64, t2276: f64, t2292: f64, t2312: f64, t3366: f64, t830: f64, t8736: f64, t8738: f64, t8741: f64, t8815: f64, t8818: f64, t8821: f64, t8824: f64, t8828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8853 = t8840 + t8852;
    let t8854 = t8853 * t810;
    let t8856 = 1.0_f64 * t788 * t8854;
    let t8857 = t3363 * t820;
    let t8862 = t1351 * t2272;
    let t8865 = t3313 * t787;
    let t8867 = 2.0_f64 * t8865 * t811;
    let t8868 = t8736 - t8738 + t8741 + 0.35089341735807877242e1_f64 * t2312 * t8815 + 6.0_f64 * t2273 * t8818 - 2.0_f64 * t8821 * t2253 - 0.11696447245269292414e1_f64 * t8824 * t2292 - t8828 - t8856 + 2.0_f64 * t8857 * t830 + 1.0_f64 * t3366 * t2268 + 0.32163958997385070134e2_f64 * t8862 * t2276 - t8867;
    (t8853, t8854, t8856, t8857, t8862, t8865, t8867, t8868)
}
