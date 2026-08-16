//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1304/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1304(t25090: f64, t7235: f64, t25803: f64, t2014: f64, t25802: f64, t7312: f64, t25866: f64, t2470: f64, t26049: f64, t7284: f64, t2453: f64, t555: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94369 = 9.0_f64 * t7235 * t25090;
    let t94371 = 3.0_f64 * t7235 * t25803;
    let t94374 = 3.0_f64 * t2014 * t7312 * t25802;
    let t94376 = 18.0_f64 * t7235 * t25866;
    let t94377 = t26049 * t2470;
    let t94378 = t7284 * t94377;
    let t94382 = t2453 * t555;
    (t94369, t94371, t94374, t94376, t94377, t94378, t94382)
}
