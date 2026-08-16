//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 886/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk886(t1041: f64, t17884: f64, t248: f64, t3051: f64, t5681: f64, t300: f64, t5769: f64, t2929: f64, t5790: f64, t1036: f64, t5905: f64, t4571: f64, t4644: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17885 = t1041 * t17884;
    let t17906 = t248 * t3051 * t5681;
    let t17907 = t1041 * t17906;
    let t17934 = t300 * t5769;
    let t17954 = t2929 * t5790;
    let t18005 = t5905 * t1036;
    let t18008 = t4644 * t4571;
    (t17885, t17906, t17907, t17934, t17954, t18005, t18008)
}
