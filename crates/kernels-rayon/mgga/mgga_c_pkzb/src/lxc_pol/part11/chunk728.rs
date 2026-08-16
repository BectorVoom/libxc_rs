//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 728/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk728(t218: f64, t220: f64, t5555: f64, t1878: f64, t679: f64, t1478: f64, t154: f64, t277: f64, t276: f64, t2045: f64, t735: f64, t2065: f64, t771: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5557 = t218 * t5555 * t220;
    let t5558 = 0.36793333333333333333e0_f64 * t5557;
    let t5560 = t218 * t1878 * t679;
    let t5589 = t154 * t1478 * t277;
    let t5591 = 5.0_f64 / 1296.0_f64 * t276 * t5589;
    let t5597 = t735 * t2045;
    let t5609 = t771 * t2065;
    (t5557, t5558, t5560, t5589, t5591, t5597, t5609)
}
