//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2007/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2007(t25338: f64, t689: f64, t887: f64, t2439: f64, t25334: f64, t7036: f64, t820: f64, t844: f64, t2751: f64, t2482: f64, t814: f64, t10782: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92930 = t689 * t25338 * t887;
    let t92935 = t2439 * t25334 * t887;
    let t92951 = t820 * t7036 * t844;
    let t92952 = t92951 * t2751;
    let t92955 = t2482 * t7036 * t814;
    let t92956 = t92955 * t10782;
    (t92930, t92935, t92951, t92952, t92955, t92956)
}
