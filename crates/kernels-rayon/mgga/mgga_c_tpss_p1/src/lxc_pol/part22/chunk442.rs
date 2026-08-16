//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 442/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk442(t30: f64, t33: f64, t1165: f64, t1322: f64, t1338: f64, t1288: f64, t490: f64, t1497: f64, t493: f64, t162: f64, zeta_threshold: f64) -> (f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1604 = 2.0_f64 * t1165 * t1338 + t1322;
    let t1608 = piecewise3(t31, 0.0_f64, 4.0_f64 / 3.0_f64 * t490 * t1288);
    let t1611 = piecewise3(t34, 0.0_f64, 4.0_f64 / 3.0_f64 * t493 * t1497);
    let t1613 = (t1608 + t1611) * t162;
    (t1604, t1613)
}
