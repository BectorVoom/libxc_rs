//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2424/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2424(t10786: f64, t18426: f64, t2747: f64, t221: f64, t2485: f64, t6022: f64, t10850: f64, t5962: f64, t775: f64, t2477: f64, t828: f64, t14718: f64, t6035: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18428 = t2747 * t18426 * t10786;
    let t18432 = t2485 * t221 * t6022;
    let t18433 = t10850 * t18432;
    let t18435 = t5962 * t775;
    let t18437 = t2477 * t828 * t18435;
    let t18440 = t14718 * t6035;
    (t18428, t18432, t18433, t18435, t18437, t18440)
}
