//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1901/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1901(t5825: f64, t999: f64, t4872: f64, t1042: f64, t1651: f64, t905: f64, t4873: f64, t3092: f64, t357: f64, t4866: f64, t4893: f64, t3117: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19700 = t5825 * t999;
    let t19701 = t4872 * t19700;
    let t19702 = t1042 * t19701;
    let t19705 = t1651 * t905;
    let t19706 = t19705 * t4873;
    let t19707 = t3092 * t19706;
    let t19716 = t357 * t4866;
    let t19717 = t4893 * t19716;
    let t19718 = t3117 * t19717;
    (t19701, t19702, t19705, t19706, t19707, t19716, t19717, t19718)
}
