//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 359/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk359(t43: f64, t50: f64, t1690: f64, t1694: f64, t47: f64, t886: f64, t478: f64, t52: f64, t893: f64, t59: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t44 = t43 <= zeta_threshold;
    let t51 = t50 <= zeta_threshold;
    let t1698 = piecewise3(t44, 0.0_f64, 4.0_f64 / 9.0_f64 * t886 * t1690 + 4.0_f64 / 3.0_f64 * t47 * t1694);
    let t1699 = t478 * t478;
    let t1702 = -t1694;
    let t1706 = piecewise3(t51, 0.0_f64, 4.0_f64 / 9.0_f64 * t893 * t1699 + 4.0_f64 / 3.0_f64 * t52 * t1702);
    let t1708 = (t1698 + t1706) * t59;
    (t1699, t1702, t1708)
}
