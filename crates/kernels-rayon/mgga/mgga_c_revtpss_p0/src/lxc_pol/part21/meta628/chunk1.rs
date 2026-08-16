//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2393/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2393(t220: f64, t40724: f64, t10777: f64, t2731: f64, t837: f64, t2482: f64, t2668: f64, t823: f64, t10782: f64, t159: f64, t33127: f64, t64: f64) -> (f64, f64, f64, f64, f64) {
    let t40725 = t40724 * t220;
    let t40728 = t10777 * t40725 * t2731 * t837;
    let t40731 = t2482 * t823 * t2668;
    let t40732 = t40731 * t10782;
    let t40735 = t64 * t33127 * t159;
    (t40725, t40728, t40731, t40732, t40735)
}
