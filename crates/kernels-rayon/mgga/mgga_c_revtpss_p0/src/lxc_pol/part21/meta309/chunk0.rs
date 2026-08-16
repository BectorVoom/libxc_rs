//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1571/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1571(t124: f64, t836: f64, t10779: f64, t2749: f64, t10777: f64, t125: f64, t2722: f64, t2723: f64, t775: f64) -> (f64, f64, f64, f64, f64) {
    let t10780 = t124 * t836;
    let t10782 = t10779 * t10780 * t2749;
    let t10783 = t10777 * t10782;
    let t10785 = t125 * t2722;
    let t10786 = t2723 * t775;
    (t10780, t10782, t10783, t10785, t10786)
}
