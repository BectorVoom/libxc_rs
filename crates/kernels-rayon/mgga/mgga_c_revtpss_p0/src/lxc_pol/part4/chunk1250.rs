//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1250/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1250(t15262: f64, t15348: f64, t15403: f64, t15516: f64, t300: f64, t3007: f64, t4724: f64, t981: f64, t3022: f64, t4734: f64, t3011: f64, t4707: f64) -> (f64, f64, f64, f64) {
    let t15519 = t300 * (t15262 + t15348 + t15403 + t15516);
    let t15520 = t4724 * t3007;
    let t15522 = 0.11696447245269292414e1_f64 * t981 * t15520;
    let t15524 = 0.34631718211362927518e2_f64 * t3022 * t4734;
    let t15525 = t3011 * t4707;
    (t15519, t15522, t15524, t15525)
}
