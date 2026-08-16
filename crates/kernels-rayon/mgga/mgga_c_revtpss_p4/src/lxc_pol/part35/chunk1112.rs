//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1112/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1112(t2482: f64, t596: f64, t7036: f64, t2681: f64, t820: f64, t25260: f64, t843: f64, t10867: f64, t64: f64, t239: f64, t7043: f64, t240: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t93034 = t2482 * t7036 * t596;
    let t93048 = t820 * t7036 * t2681;
    let t93054 = t820 * t25260 * t843;
    let t93060 = t10867 * t64;
    let t93062 = t820 * t93060 * t239;
    let t93066 = t820 * t7043 * t2681;
    let t93072 = t2482 * t7043 * t596;
    let t93082 = t25260 * t240;
    (t93034, t93048, t93054, t93062, t93066, t93072, t93082)
}
