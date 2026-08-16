//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1063/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1063(t121305: f64, t32186: f64, t786: f64, t32216: f64, t4075: f64, t122: f64, t32219: f64, t3916: f64, t119833: f64, t121245: f64, t121248: f64, t121239: f64, t25875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t121307 = t786 * t121305 * t32186;
    let t121308 = 0.20077843028252776532e-3_f64 * t121307;
    let t121309 = t32216 * t4075;
    let t121310 = t786 * t121309;
    let t121312 = t32219 * t122 * t3916;
    let t121313 = t121310 * t121312;
    let t121326 = t119833 * t121245;
    let t121327 = t121326 * t121248;
    let t121333 = t25875 * t121239;
    (t121308, t121309, t121310, t121312, t121313, t121326, t121327, t121333)
}
