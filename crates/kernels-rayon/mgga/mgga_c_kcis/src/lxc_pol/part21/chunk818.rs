//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 818/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk818(t3132: f64, t738: f64, t3136: f64, t743: f64, t3139: f64, t733: f64, t1080: f64, t2475: f64, t3116: f64, t3119: f64, t3124: f64, t3127: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10021 = t738 * t3132;
    let t10026 = t743 * t3136;
    let t10028 = t733 * t3139;
    let t10033 = t2475 * t1080;
    let t10035 = t733 * t3116;
    let t10037 = t733 * t3119;
    let t10045 = t738 * t3124;
    let t10048 = t738 * t3127;
    (t10021, t10026, t10028, t10033, t10035, t10037, t10045, t10048)
}
