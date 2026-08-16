//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 917/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk917(t19688: f64, t4994: f64, t13181: f64, t1713: f64, t1020: f64, t1662: f64, t4818: f64, t14072: f64, t3200: f64, t4823: f64, t9517: f64, t6491: f64, t922: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t19689 = t4994 * t19688;
    let t19691 = t13181 * t1713;
    let t19692 = t1020 * t19691;
    let t19694 = t1662 * t4818;
    let t19695 = t14072 * t19694;
    let t19696 = t3200 * t19695;
    let t19698 = t1662 * t4823;
    let t19699 = t9517 * t19698;
    let t19700 = t3200 * t19699;
    let t19702 = t6491 * t922;
    (t19689, t19692, t19694, t19696, t19698, t19700, t19702)
}
