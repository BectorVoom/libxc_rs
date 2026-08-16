//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 899/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk899(t1036: f64, t5905: f64, t4571: f64, t4644: f64, t1009: f64, t5848: f64, t1011: f64, t1019: f64, t10422: f64, t5908: f64, t3070: f64, t225: f64, t5915: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t18005 = t5905 * t1036;
    let t18008 = t4644 * t4571;
    let t18028 = t5848 * t1009;
    let t18029 = t18028 * t1011;
    let t18030 = t18029 * t1019;
    let t18041 = t10422 * t5908;
    let t18042 = t3070 * t18041;
    let t18074 = t5915 * t225;
    (t18005, t18008, t18028, t18030, t18041, t18042, t18074)
}
