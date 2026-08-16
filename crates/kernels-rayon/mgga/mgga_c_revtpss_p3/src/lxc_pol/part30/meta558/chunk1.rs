//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2000/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2000(t2751: f64, t92951: f64, t2482: f64, t7036: f64, t814: f64, t10782: f64, t10744: f64, t2664: f64, t7028: f64, t25240: f64, t2693: f64, t2710: f64) -> (f64, f64, f64, f64, f64) {
    let t92952 = t92951 * t2751;
    let t92955 = t2482 * t7036 * t814;
    let t92956 = t92955 * t10782;
    let t92963 = t10744 * t7028 * t2664;
    let t92966 = t2710 * t25240 * t2693;
    (t92952, t92955, t92956, t92963, t92966)
}
