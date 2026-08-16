//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1054/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1054(t124: f64, t836: f64, t10779: f64, t2749: f64, t10777: f64, t820: f64, t823: f64, t844: f64, t2751: f64, t2681: f64, t839: f64, t222: f64, t9727: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10780 = t124 * t836;
    let t10782 = t10779 * t10780 * t2749;
    let t10783 = t10777 * t10782;
    let t10811 = t820 * t823 * t844;
    let t10812 = t10811 * t2751;
    let t10815 = t820 * t823 * t2681;
    let t10816 = t10815 * t839;
    let t10824 = 455.0_f64 / 1296.0_f64 * t9727 * t222;
    (t10783, t10811, t10812, t10815, t10816, t10824)
}
