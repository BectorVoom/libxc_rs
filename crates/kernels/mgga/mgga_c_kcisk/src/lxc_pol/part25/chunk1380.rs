//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1380/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1380<F: Float>(t116996: F, t33167: F, t34416: F, t34452: F, t9736: F, t118184: F, t7233: F, t33227: F, t6758: F, t112576: F, t113123: F, t113181: F, t116998: F, t117001: F, t117004: F, t117008: F, t117699: F, t117725: F, t118187: F, t2803: F, t2807: F, t33196: F, t60929: F, t79: F) -> (F,) {
    let t118343 = 0.23214722222222222222e-2 * t116996;
    let t118348 = 0.11574074074074074074e-2 * t34416 * t33167;
    let t118355 = 0.34722222222222222222e-2 * t34452 * t9736;
    let t118360 = t7233 * t118184;
    let t118362 = t118360 * t6758 * t33227;
    let t118368 = -0.26805555555555555556e-2 * t113123 * t118187 - t118343 + 0.61905925925925925924e-2 * t116998 - 0.92858888888888888886e-2 * t117001 + 0.10317654320987654321e-2 * t117004 - t118348 - 0.60312500000000000001e-2 * t33196 * t117699 - 0.20104166666666666667e-2 * t33196 * t117725 + 0.38691203703703703703e-3 * t117008 - t118355 - 0.52083333333333333333e-2 * t60929 * t79 * t2803 * t2807 + 0.1787037037037037037e-2 * t113123 * t118362 + 0.46296296296296296296e-2 * t113181 * t118362 - 0.77382407407407407406e-3 * t112576;
    (t118368,)
}
