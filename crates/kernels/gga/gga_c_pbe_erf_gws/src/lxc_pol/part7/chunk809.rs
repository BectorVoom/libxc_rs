//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 809/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk809<F: Float>(t1673: F, t1680: F, t5373: F, t636: F, t422: F, t661: F, t1416: F, t7115: F, t7505: F, t4892: F, t5218: F, t5220: F, t5529: F, t5544: F, t562: F, t7068: F) -> (F, F, F, F, F, F, F) {
    let t16757 = t1680 * t1673;
    let t16758 = 16.0 / 45.0 * t16757;
    let t16759 = t5373 * t636;
    let t16760 = 16.0 / 45.0 * t16759;
    let t16761 = t422 * t661;
    let t16762 = t16761 * t1416;
    let t16765 = 32.0 / 15.0 * t7115 * t7505 * t16762;
    let t16768 = 32.0 / 15.0 * t5218 * t5220 * t4892;
    let t16771 = 32.0 / 15.0 * t5218 * t5220 * t5529;
    let t16775 = 32.0 / 9.0 * t5218 * t7068 * t562 * t5544;
    (t16758, t16760, t16762, t16765, t16768, t16771, t16775)
}
