//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1392/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1392(t1036: f64, t23551: f64, t23562: f64, t343: f64, t83032: f64, t210: f64, t23322: f64, t23460: f64, t995: f64, t3: f64, t9258: f64, t23452: f64, t6739: f64, t6741: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t83082 = t23551 * t1036;
    let t83085 = t23562 * t83032 * t343;
    let t83092 = t23322 * t210;
    let t83098 = t23460 * t995;
    let t83100 = t3 * t9258;
    let t83111 = t23452 * t6739 * t6741;
    (t83082, t83085, t83092, t83098, t83100, t83111)
}
