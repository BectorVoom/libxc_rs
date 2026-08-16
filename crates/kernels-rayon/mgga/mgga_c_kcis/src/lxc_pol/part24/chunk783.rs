//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 783/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk783(t2822: f64, t5000: f64, t251: f64, t691: f64, t1018: f64, t86: f64, t4996: f64, t4989: f64, t1131: f64, t3209: f64, t4904: f64, t743: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13391 = t2822 * t5000;
    let t13396 = t691 * t251;
    let t13398 = t86 * t13396 * t1018;
    let t13399 = t13398 * t4996;
    let t13408 = t2822 * t4989;
    let t13409 = 0.22109259259259259258e-2_f64 * t13408;
    let t13410 = t3209 * t1131;
    let t13472 = 0.4705225e-4_f64 * t743 * t4904;
    (t13391, t13398, t13399, t13408, t13409, t13410, t13472)
}
