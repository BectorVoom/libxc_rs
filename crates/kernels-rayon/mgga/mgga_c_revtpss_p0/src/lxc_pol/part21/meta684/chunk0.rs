//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2499/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2499(t371: f64, t481: f64, t482: f64, t9291: f64, t12627: f64, t1284: f64, t3624: f64, t12910: f64, t12911: f64, t12916: f64, t12640: f64, t127: f64, t12866: f64, t3630: f64, t3712: f64) -> (f64, f64, f64, f64, f64) {
    let t44607 = 0.14820648238345094262e-3_f64 * t481 * t371 * t9291 * t482;
    let t44609 = t12627 * t1284 * t3624;
    let t44616 = t12910 * t12916 * t12911;
    let t44624 = t12640 * t1284 * t3624;
    let t44634 = t12866 * t127 * t3712 * t3630;
    (t44607, t44609, t44616, t44624, t44634)
}
