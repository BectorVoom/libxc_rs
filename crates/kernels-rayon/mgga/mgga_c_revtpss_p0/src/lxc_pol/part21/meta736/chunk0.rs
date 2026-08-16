//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2586/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2586(t47530: f64, t9682: f64, t2439: f64, t3895: f64, t4132: f64, t1357: f64, t689: f64, t9659: f64, t3899: f64, t10175: f64, t9671: f64, t10146: f64, t123: f64, t3915: f64, t676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47531 = t47530 * t9682;
    let t47534 = t2439 * t3895 * t4132;
    let t47537 = t689 * t1357 * t9659;
    let t47540 = t689 * t3899 * t4132;
    let t47550 = t10175 * t9671;
    let t47554 = t3915 * t123 * t676 * t10146;
    (t47531, t47534, t47537, t47540, t47550, t47554)
}
