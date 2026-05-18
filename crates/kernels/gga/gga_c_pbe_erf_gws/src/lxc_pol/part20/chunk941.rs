//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 941/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk941<F: Float>(t10559: F, t10591: F, t650: F, t186: F, t211: F, t7421: F, t1033: F, t2724: F, t7460: F, t10474: F, t10476: F, t10478: F, t10480: F, t10484: F, t10487: F, t10491: F, t10495: F, t10497: F, t10499: F, t10504: F, t10509: F, t10512: F) -> (F, F, F, F, F) {
    let t10592 = t10559 + t10591;
    let t10593 = t650 * t10592;
    let t10594 = t186 * t10593;
    let t10596 = F::new(2.0) / F::new(15.0) * t211 * t10594;
    let t10597 = F::new(8.0) / F::new(135.0) * t7421;
    let t10599 = F::new(4.0) / F::new(15.0) * t1033 * t2724;
    let t10600 = F::new(16.0) / F::new(405.0) * t7460;
    let t10601 = t10474 - t10476 - t10478 - t10480 - t10484 + t10487 - t10491 - t10495 - t10497 + t10499 + t10504 - t10509 + t10512 - t10596 - t10597 - t10599 - t10600;
    (t10596, t10597, t10599, t10600, t10601)
}
