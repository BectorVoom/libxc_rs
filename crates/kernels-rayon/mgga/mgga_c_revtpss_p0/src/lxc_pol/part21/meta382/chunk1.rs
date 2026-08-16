//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1803/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1803(t12487: f64, t3523: f64, t1175: f64, t3495: f64, t1188: f64, t1189: f64, t3515: f64, t1187: f64, t1170: f64, t3471: f64, t1168: f64, t3479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12488 = t12487 * t3523;
    let t12491 = t1175 * t3495;
    let t12494 = t12487 * t1188;
    let t12497 = t1189 * t3515;
    let t12500 = t3515 * t3523;
    let t12501 = t12500 * t1187;
    let t12504 = t1170 * t3471;
    let t12508 = t3471 * t3479 * t1168;
    (t12488, t12491, t12494, t12497, t12500, t12501, t12504, t12508)
}
