//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1468/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1468(t3801: f64, t5501: f64, t12587: f64, t1832: f64, t1298: f64, t16786: f64, t16788: f64, t16790: f64, t16809: f64, t16814: f64, t16834: f64, t16837: f64, t16839: f64, t16842: f64, t16844: f64, t16846: f64, t16945: f64, t17094: f64, t17160: f64, t17162: f64, t17166: f64, t17168: f64, t3794: f64, t3798: f64, t5023: f64, t5505: f64) -> f64 {
    let t18128 = t5501 * t3801;
    let t18134 = t1832 * t12587;
    let t18138 = -2.0_f64 * t1298 * t18128 * t5023 + 2.0_f64 * t18134 * t3798 * t5023 - t3794 * t5023 * t5505 - t16786 - t16788 - t16790 - t16809 - t16814 + t16834 + t16837 + t16839 + t16842 + t16844 + t16846 + t16945 - t17094 + t17160 + t17162 - t17166 - t17168;
    t18138
}
