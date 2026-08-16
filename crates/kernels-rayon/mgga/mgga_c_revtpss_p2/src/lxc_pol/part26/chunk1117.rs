//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1117/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1117(t10773: f64, t25270: f64, t10766: f64, t10794: f64, t7036: f64, t820: f64, t844: f64, t2751: f64, t2482: f64, t814: f64, t10782: f64, t10803: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92944 = t25270 * t10773;
    let t92946 = t25270 * t10766;
    let t92948 = t25270 * t10794;
    let t92951 = t820 * t7036 * t844;
    let t92952 = t92951 * t2751;
    let t92955 = t2482 * t7036 * t814;
    let t92956 = t92955 * t10782;
    let t92958 = t25270 * t10803;
    (t92944, t92946, t92948, t92952, t92956, t92958)
}
