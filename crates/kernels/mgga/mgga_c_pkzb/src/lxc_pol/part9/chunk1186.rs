//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1186/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1186<F: Float>(t237: F, t7510: F, t732: F, t1987: F, t7528: F, t1306: F, t20636: F, t20641: F, t20642: F, t20647: F, t20649: F, t20652: F, t20654: F, t20658: F, t20662: F, t7543: F) -> (F, F, F) {
    let t20663 = t237 * t7510;
    let t20665 = F::cast_from(0.17544670867903938621e1_f64) * t20663 * t732;
    let t20667 = F::cast_from(0.70178683471615754484e1_f64) * t1987 * t7528;
    let t20668 = F::cast_from(6.0_f64) * t1306 * t20642 * t7543 - t20636 + t20641 + t20647 + t20649 + t20652 + t20654 - t20658 + t20662 - t20665 + t20667;
    (t20665, t20667, t20668)
}
