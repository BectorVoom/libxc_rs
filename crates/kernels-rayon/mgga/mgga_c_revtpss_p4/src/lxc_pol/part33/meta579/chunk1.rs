//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1990/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1990(t2439: f64, t25334: f64, t887: f64, t7036: f64, t820: f64, t844: f64, t2482: f64, t814: f64, t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64) -> (f64, f64, f64, f64, f64) {
    let t92935 = t2439 * t25334 * t887;
    let t92951 = t820 * t7036 * t844;
    let t92955 = t2482 * t7036 * t814;
    let t92963 = t10744 * t7028 * t2664;
    let t92966 = t2710 * t25240 * t2693;
    (t92935, t92951, t92955, t92963, t92966)
}
