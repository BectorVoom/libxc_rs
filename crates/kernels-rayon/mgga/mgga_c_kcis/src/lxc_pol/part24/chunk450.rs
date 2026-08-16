//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 450/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk450(t3110: f64, t335: f64, t333: f64, t1057: f64, t733: f64, t1065: f64, t738: f64, t1080: f64, t743: f64, t113: f64, t2844: f64, t3054: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3111 = t3110 * t335;
    let t3113 = 0.16804375e-4_f64 * t333 * t3111;
    let t3114 = t733 * t1057;
    let t3122 = t738 * t1065;
    let t3130 = t743 * t1080;
    let t3150 = t113 * t2844;
    let t3153 = 0.23911438650126355246e-1_f64 * t3054;
    (t3113, t3114, t3122, t3130, t3150, t3153)
}
