//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 449/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk449(t30: f64, t33: f64, t1857: f64, t512: f64, t1856: f64, t187: f64, t1344: f64, t1468: f64, t1348: f64, t1711: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t1858 = t512 * t1857;
    let t1860 = 0.19751673498613801407e-1_f64 * t1856 * t187;
    let t1863 = piecewise3(t31, 0.0_f64, 2.0_f64 / 3.0_f64 * t1344 * t1468);
    let t1866 = piecewise3(t34, 0.0_f64, 2.0_f64 / 3.0_f64 * t1348 * t1711);
    let t1868 = t1863 / 2.0_f64 + t1866 / 2.0_f64;
    (t1858, t1860, t1868)
}
