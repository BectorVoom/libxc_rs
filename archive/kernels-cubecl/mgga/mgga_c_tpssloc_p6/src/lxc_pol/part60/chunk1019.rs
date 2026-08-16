//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 1019/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk1019<F: Float>(t28813: F, t8607: F, t27188: F, t7468: F, t33234: F, t28045: F, t7042: F, t33358: F, t91655: F, t33363: F, t7754: F, t2018: F, t26161: F, t26558: F, t6463: F) -> (F, F, F, F, F, F, F) {
    let t128377 = F::cast_from(2.0_f64) * t8607 * t28813;
    let t128381 = F::cast_from(4.0_f64) * t27188 * t7468;
    let t128383 = F::cast_from(4.0_f64) * t33234 * t7468;
    let t128385 = F::cast_from(4.0_f64) * t7042 * t28045;
    let t128387 = F::cast_from(6.0_f64) * t91655 * t33358;
    let t128393 = F::cast_from(2.0_f64) * t33363 * t7754;
    let t128397 = F::cast_from(2.0_f64) * t26161 * t26558 * t2018 * t6463;
    (t128377, t128381, t128383, t128385, t128387, t128393, t128397)
}
