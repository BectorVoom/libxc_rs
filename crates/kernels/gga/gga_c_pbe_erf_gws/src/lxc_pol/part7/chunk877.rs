//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 877/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk877<F: Float>(t16762: F, t7115: F, t7505: F, t4892: F, t5218: F, t5220: F, t5529: F, t5544: F, t562: F, t7068: F, t5275: F, t579: F) -> (F, F, F, F, F) {
    let t16765 = F::new(32.0) / F::new(15.0) * t7115 * t7505 * t16762;
    let t16768 = F::new(32.0) / F::new(15.0) * t5218 * t5220 * t4892;
    let t16771 = F::new(32.0) / F::new(15.0) * t5218 * t5220 * t5529;
    let t16775 = F::new(32.0) / F::new(9.0) * t5218 * t7068 * t562 * t5544;
    let t16777 = F::new(8.0) / F::new(15.0) * t579 * t5275;
    (t16765, t16768, t16771, t16775, t16777)
}
