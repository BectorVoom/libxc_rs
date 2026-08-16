//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1127/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1127(t14098: f64, t4582: f64, t3121: f64, t4593: f64, t3041: f64, t1031: f64, t4616: f64, t1612: f64, t3082: f64, t1025: f64, t1041: f64, t1046: f64, t10873: f64, t10883: f64, t10952: f64, t10965: f64, t14077: f64, t14080: f64, t14084: f64, t14085: f64, t14093: f64, t1622: f64, t3039: f64, t3048: f64, t3117: f64, t378: f64, t4585: f64, t4590: f64, t4600: f64, t4636: f64) -> f64 {
    let t14099 = t4582 * t14098;
    let t14102 = t4593 * t3121;
    let t14103 = t4582 * t14102;
    let t14106 = t4593 * t3041;
    let t14107 = t4582 * t14106;
    let t14114 = t4616 * t1031;
    let t14117 = t1612 * t3082;
    let t14120 = -t14077 * t1025 / 288.0_f64 - t14080 * t1046 / 432.0_f64 + t14084 + t14085 * t1046 / 2304.0_f64 + t10965 * t1622 / 4608.0_f64 + t3117 * t4636 / 2304.0_f64 + t1041 * t14093 / 4608.0_f64 - t10952 * t4600 / 1536.0_f64 - t3039 * t14099 / 1536.0_f64 - t3039 * t14103 / 3072.0_f64 + t10883 * t14107 / 3072.0_f64 + t3048 * t4585 / 216.0_f64 - 5.0_f64 / 1296.0_f64 * t3048 * t4590 - t14114 * t378 / 288.0_f64 - t14117 / 13824.0_f64 - t10873 / 648.0_f64;
    t14120
}
