//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1184/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1184<F: Float>(t32661: F, t34985: F, t1774: F, t6681: F, t7298: F, t1526: F, t6673: F, t7705: F, t342: F, t35000: F, t630: F, t11280: F, t1349: F, t138586: F, t138598: F, t138607: F, t138611: F, t138625: F, t138629: F, t138635: F, t26579: F, t26768: F, t27043: F, t27097: F, t27106: F, t27114: F, t32658: F, t343: F, t34994: F, t356: F, t379: F, t461: F, t5772: F, t5843: F, t6616: F, t7150: F, t72: F, t7299: F, t925: F) -> F {
    let t149550 = t34985 * t32661;
    let t149553 = t7298 * t1774 * t6681;
    let t149569 = t1526 * t7705 * t6673;
    let t149586 = t342 * t630 * t35000;
    let t149593 = t149550 / F::cast_from(18.0_f64) + t149553 / F::cast_from(18.0_f64) + t1349 * t356 * t6616 * t379 / F::cast_from(18.0_f64) - t1526 * t11280 * t27043 / F::cast_from(6.0_f64) - t7298 * t461 * t27114 / F::cast_from(6.0_f64) - t26579 * t7150 * t7299 / F::cast_from(6.0_f64) - t149569 / F::cast_from(36.0_f64) - t138586 / F::cast_from(36.0_f64) + t138598 / F::cast_from(18.0_f64) + t138607 / F::cast_from(18.0_f64) - t5772 * t27097 / F::cast_from(9.0_f64) - t5772 * t27106 / F::cast_from(9.0_f64) + t1349 * t356 * t5843 * t925 / F::cast_from(18.0_f64) - t138611 - t32658 * t34994 / F::cast_from(6.0_f64) - t138625 / F::cast_from(9.0_f64) + t138629 - t149586 / F::cast_from(12.0_f64) - t138635 / F::cast_from(12.0_f64) - t342 * t343 * t72 * t26768 / F::cast_from(4.0_f64);
    t149593
}
