//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1184/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1184(t32661: f64, t34985: f64, t1774: f64, t6681: f64, t7298: f64, t1526: f64, t6673: f64, t7705: f64, t342: f64, t35000: f64, t630: f64, t11280: f64, t1349: f64, t138586: f64, t138598: f64, t138607: f64, t138611: f64, t138625: f64, t138629: f64, t138635: f64, t26579: f64, t26768: f64, t27043: f64, t27097: f64, t27106: f64, t27114: f64, t32658: f64, t343: f64, t34994: f64, t356: f64, t379: f64, t461: f64, t5772: f64, t5843: f64, t6616: f64, t7150: f64, t72: f64, t7299: f64, t925: f64) -> f64 {
    let t149550 = t34985 * t32661;
    let t149553 = t7298 * t1774 * t6681;
    let t149569 = t1526 * t7705 * t6673;
    let t149586 = t342 * t630 * t35000;
    let t149593 = t149550 / 18.0_f64 + t149553 / 18.0_f64 + t1349 * t356 * t6616 * t379 / 18.0_f64 - t1526 * t11280 * t27043 / 6.0_f64 - t7298 * t461 * t27114 / 6.0_f64 - t26579 * t7150 * t7299 / 6.0_f64 - t149569 / 36.0_f64 - t138586 / 36.0_f64 + t138598 / 18.0_f64 + t138607 / 18.0_f64 - t5772 * t27097 / 9.0_f64 - t5772 * t27106 / 9.0_f64 + t1349 * t356 * t5843 * t925 / 18.0_f64 - t138611 - t32658 * t34994 / 6.0_f64 - t138625 / 9.0_f64 + t138629 - t149586 / 12.0_f64 - t138635 / 12.0_f64 - t342 * t343 * t72 * t26768 / 4.0_f64;
    t149593
}
