//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1008/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1008(t1609: f64, t19330: f64, t2924: f64, t1622: f64, t6173: f64, t11452: f64, t23705: f64, t23451: f64, t3014: f64, t11574: f64, t15189: f64, t18919: f64, t18924: f64, t18934: f64, t23479: f64, t23483: f64, t23487: f64, t23490: f64, t23501: f64, t23505: f64) -> (f64, f64, f64, f64, f64) {
    let t23770 = t19330 * t1609;
    let t23772 = 0.48245938496077605201e2_f64 * t2924 * t23770;
    let t23773 = t1622 * t6173;
    let t23776 = t23705 * t11452;
    let t23785 = t23451 * t3014;
    let t23798 = -t11574 - 0.2283111111111111111e-1_f64 * t15189 + 0.11415555555555555555e-1_f64 * t18919 - 0.34246666666666666665e-1_f64 * t18924 + 0.17123333333333333333e-1_f64 * t18934 - 0.19025925925925925925e-1_f64 * t23479 + 0.68493333333333333331e-1_f64 * t23483 - 0.34246666666666666665e-1_f64 * t23501 - 0.10274e0_f64 * t23487 + 0.10274e0_f64 * t23505 - 0.17123333333333333333e-1_f64 * t23490;
    (t23772, t23773, t23776, t23785, t23798)
}
