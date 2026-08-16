//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1731/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1731(t25826: f64, t28036: f64, t4287: f64, t6998: f64, t4237: f64, t76: f64, t13269: f64, t38: f64, t1497: f64, t640: f64, t77: f64, t4241: f64, t84: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28037 = t25826 * t28036;
    let t28039 = t6998 * t4287;
    let t28089 = t76 * t4237;
    let t28093 = t13269 * t38;
    let t28104 = t640 * t1497;
    let t28105 = t77 * t28104;
    let t28108 = t84 * t4241;
    (t28037, t28039, t28089, t28093, t28105, t28108)
}
