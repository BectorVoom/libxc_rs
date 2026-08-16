//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 444/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk444(t30: f64, t33: f64, t1614: f64, t489: f64, t1613: f64, t187: f64, t1197: f64, t1288: f64, t1201: f64, t1497: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1615 = t489 * t1614;
    let t1617 = 0.19751673498613801407e-1_f64 * t1613 * t187;
    let t1620 = piecewise3(t31, 0.0_f64, 2.0_f64 / 3.0_f64 * t1197 * t1288);
    let t1623 = piecewise3(t34, 0.0_f64, 2.0_f64 / 3.0_f64 * t1201 * t1497);
    let t1625 = t1620 / 2.0_f64 + t1623 / 2.0_f64;
    (t1615, t1617, t1625)
}
