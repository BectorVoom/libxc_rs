//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1748/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1748(t239: f64, t2247: f64, t607: f64, t1927: f64, t644: f64, t531: f64, t7311: f64, t1962: f64, t198: f64, t206: f64) -> (f64, f64, f64, f64, f64) {
    let t25137 = 88.0_f64 / 9.0_f64 * t239;
    let t25162 = t2247 * t607;
    let t25163 = t1927 * t644;
    let t25190 = t531 * t7311;
    let t25206 = t198 * t206 * t1962;
    (t25137, t25162, t25163, t25190, t25206)
}
