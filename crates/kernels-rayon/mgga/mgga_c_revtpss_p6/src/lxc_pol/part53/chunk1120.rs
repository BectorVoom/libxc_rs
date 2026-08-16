//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1120/1244 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1120(t121241: f64, t121333: f64, t121116: f64, t32208: f64, t121309: f64, t7063: f64, t121312: f64, t121305: f64, t32186: f64, t119900: f64, t121165: f64, t240: f64, t545: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t121334 = t121333 * t121241;
    let t121336 = t121116 * t32208;
    let t121338 = t7063 * t121309;
    let t121339 = t121338 * t121312;
    let t121342 = t7063 * t121305 * t32186;
    let t121343 = 0.35698404904233436678e-3_f64 * t121342;
    let t121346 = t119900 * t545 * t240 * t121165;
    (t121334, t121336, t121338, t121339, t121343, t121346)
}
