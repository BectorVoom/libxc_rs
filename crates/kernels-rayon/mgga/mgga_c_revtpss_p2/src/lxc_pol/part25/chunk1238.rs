//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1238/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1238(t7030: f64, t9784: f64, t10788: f64, t27261: f64, t2482: f64, t25260: f64, t27: f64, t10852: f64, t25266: f64, t2756: f64, t10836: f64, t25227: f64, t2661: f64) -> (f64, f64, f64, f64, f64) {
    let t93020 = t9784 * t7030;
    let t93021 = 0.14450132032386466905e-2_f64 * t93020;
    let t93022 = t27261 * t10788;
    let t93025 = t2482 * t25260 * t27;
    let t93026 = t93025 * t10852;
    let t93028 = t25266 * t2756;
    let t93031 = t2661 * t25227 * t10836;
    (t93021, t93022, t93026, t93028, t93031)
}
