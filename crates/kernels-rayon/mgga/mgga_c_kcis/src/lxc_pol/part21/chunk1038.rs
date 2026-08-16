//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1038/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1038(t169: f64, t740: f64, t9323: f64, t234: f64, t1767: f64, t3217: f64, t1262: f64, t1851: f64, t2153: f64, t2539: f64, t9275: f64, t1295: f64, t914: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t170 = t169 <= zeta_threshold;
    let t18374 = 2.0_f64 * t740;
    let t18375 = 6.0_f64 * t9323;
    let t18376 = -t18374 + t18375;
    let t18401 = piecewise3(t170, 0.0_f64, -t18376);
    let t18402 = t234 * t18401;
    let t19575 = t3217 * t1767;
    let t20572 = t1851 * t1262;
    let t26390 = t2153 * t2539;
    let t26391 = t9275 * t26390;
    let t26392 = 6.0_f64 * t26391;
    let t26393 = t914 * t1295;
    (t18401, t18402, t19575, t20572, t26390, t26391, t26392, t26393)
}
