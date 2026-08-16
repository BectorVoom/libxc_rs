//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 905/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk905(t1149: f64, t1733: f64, t3384: f64, t1723: f64, t3390: f64, t1134: f64, t3358: f64, t3394: f64, t5044: f64, t5049: f64, t5054: f64, t5058: f64) -> (f64, f64, f64, f64, f64) {
    let t5068 = t1733 * t1149;
    let t5070 = 2.0_f64 * t3384 * t5068;
    let t5071 = t3390 * t1723;
    let t5072 = t5071 * t1134;
    let t5079 = t3394 - t3358 / 9.0_f64 - t5044 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t5049 + 2.0_f64 / 3.0_f64 * t5054 + t5058 / 3.0_f64;
    (t5068, t5070, t5071, t5072, t5079)
}
