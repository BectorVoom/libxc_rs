//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2117/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2117(t25188: f64, t7935: f64, t2014: f64, t25802: f64, t7934: f64, t28167: f64, t35070: f64, t5627: f64, t25081: f64, t7897: f64, t25083: f64, t28020: f64, t7315: f64) -> (f64, f64, f64, f64, f64) {
    let t98440 = t25188 * t7935;
    let t98442 = t2014 * t7934 * t25802;
    let t98449 = 12.0_f64 * t28167 * t35070 * t5627;
    let t98450 = t7897 * t25081;
    let t98452 = 6.0_f64 * t98450 * t25083;
    let t98455 = 2.0_f64 * t2014 * t28020 * t7315;
    (t98440, t98442, t98449, t98452, t98455)
}
