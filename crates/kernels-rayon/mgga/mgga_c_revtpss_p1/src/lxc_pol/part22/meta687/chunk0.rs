//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2677/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2677(t30: f64, t21881: f64, t508: f64, t1518: f64, t5517: f64, t13584: f64, t9375: f64, t6785: f64, t9335: f64, t3833: f64, t5824: f64, t18280: f64, t2255: f64, t513: f64, t5549: f64, t605: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t21882 = t508 * t21881;
    let t21891 = t5517 * t1518;
    let t21901 = 40.0_f64 * t13584;
    let t21905 = 0.5848223622634646207e0_f64 * t9375;
    let t21906 = t9335 * t6785;
    let t21911 = t3833 * t5824;
    let t21917 = piecewise3(t31, 0.0_f64, -8.0_f64 / 27.0_f64 * t21906 * t605 + 16.0_f64 / 9.0_f64 * t5549 * t2255 + 4.0_f64 / 9.0_f64 * t21911 * t605 + 4.0_f64 / 3.0_f64 * t513 * t18280);
    (t21882, t21891, t21901, t21905, t21906, t21911, t21917)
}
