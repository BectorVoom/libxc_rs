//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 592/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk592(t1065: f64, t738: f64, t1064: f64, t2829: f64, t2845: f64, t945: f64, t1080: f64, t743: f64, t2850: f64, t104: f64, t111: f64, t3105: f64, t3109: f64, t3113: f64, t3114: f64, t3116: f64, t3119: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3122 = t738 * t1065;
    let t3124 = t1064 * t2829;
    let t3127 = t945 * t2845;
    let t3130 = t743 * t1080;
    let t3132 = t1064 * t2850;
    let t3135 = t3105 - t3109 - t3113 + 0.9368e-2_f64 * t3114 - 0.3513e-2_f64 * t104 * t3116 + 0.1171e-2_f64 * t104 * t3119 - 0.26416666666666666666e-2_f64 * t3122 + 0.7925e-3_f64 * t111 * t3124 - 0.52833333333333333333e-3_f64 * t111 * t3127 - 0.23526125e-4_f64 * t3130 - 0.1585e-2_f64 * t111 * t3132;
    (t3122, t3124, t3127, t3130, t3132, t3135)
}
