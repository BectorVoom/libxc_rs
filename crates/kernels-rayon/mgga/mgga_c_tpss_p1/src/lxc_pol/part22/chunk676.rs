//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 676/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk676(t30: f64, t33: f64, t14: f64, t22: f64, t498: f64, t558: f64, t563: f64, t491: f64, t580: f64, t1197: f64, t1991: f64, t494: f64, t1006: f64, t1201: f64, t2829: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3211 = t14 * t22;
    let t3213 = 12.0_f64 * t3211 * t498;
    let t3214 = t558 * t563;
    let t3216 = 32.0_f64 * t3214 * t498;
    let t3217 = 1.0_f64 / t491;
    let t3218 = t580 * t580;
    let t3224 = piecewise3(t31, 0.0_f64, -2.0_f64 / 9.0_f64 * t3217 * t3218 + 2.0_f64 / 3.0_f64 * t1197 * t1991);
    let t3225 = 1.0_f64 / t494;
    let t3226 = t1006 * t1006;
    let t3232 = piecewise3(t34, 0.0_f64, -2.0_f64 / 9.0_f64 * t3225 * t3226 + 2.0_f64 / 3.0_f64 * t1201 * t2829);
    (t3211, t3213, t3214, t3216, t3217, t3218, t3224, t3225, t3226, t3232)
}
