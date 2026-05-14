//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1203/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1203<F: Float>(t101767: F, t101744: F, t101751: F, t101754: F, t101758: F, t101761: F, t101764: F, t93521: F, t93523: F, t93530: F, t93541: F, t25904: F, t376: F, t89: F, t28: F, t93468: F, t942: F) -> (F, F, F, F) {
    let t101768 = t101767 / 18.0;
    let t101769 = t101744 / 9.0 + t93521 / 54.0 + 2.0 / 27.0 * t93523 + t93530 / 3.0 - t93541 / 9.0 + 2.0 / 9.0 * t101751 - 2.0 / 9.0 * t101754 - t101758 / 18.0 - 8.0 / 9.0 * t101761 + 8.0 / 27.0 * t101764 - t101768;
    let t101771 = t89 * t376 * t25904;
    let t101772 = 4.0 / 9.0 * t101771;
    let t101775 = t89 * t28 * t93468 * t942;
    (t101769, t101771, t101772, t101775)
}
