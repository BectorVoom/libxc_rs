//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3645/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3645(t3385: f64, t3433: f64, t6471: f64, t1130: f64, t20469: f64, t1151: f64, t20629: f64, t3428: f64, t3432: f64, t6433: f64, t3436: f64, t1733: f64, t58460: f64) -> (f64, f64, f64, f64, f64) {
    let t68946 = 6.0_f64 * t3433 * t6471 * t3385;
    let t68947 = t20469 * t1130;
    let t68949 = 2.0_f64 * t68947 * t1151;
    let t68951 = 1.0_f64 * t20629 * t3428;
    let t68952 = t6433 * t3432;
    let t68954 = 0.16081979498692535067e2_f64 * t68952 * t3436;
    let t68956 = 2.0_f64 * t58460 * t1733;
    (t68946, t68949, t68951, t68954, t68956)
}
