//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2118/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2118(t25082: f64, t49582: f64, t8717: f64, t2014: f64, t25089: f64, t28172: f64, t27154: f64, t95088: f64, t26089: f64, t5542: f64, t2322: f64, t28043: f64) -> (f64, f64, f64, f64, f64) {
    let t98458 = 3.0_f64 * t25082 * t8717 * t49582;
    let t98461 = 3.0_f64 * t2014 * t28172 * t25089;
    let t98463 = 6.0_f64 * t95088 * t27154;
    let t98467 = t2014 * t26089 * t5542;
    let t98472 = 4.0_f64 * t2322 * t28043;
    (t98458, t98461, t98463, t98467, t98472)
}
