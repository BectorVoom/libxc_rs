//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3638/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3638<F: Float>(t43911: F, t56176: F, t56183: F, t56185: F, t68342: F, t68347: F, t68350: F, t68353: F, t68357: F, t68360: F, t68363: F, t68366: F, t68368: F, t68370: F, t68373: F) -> F {
    let t68854 = F::cast_from(0.33218518518518518518e0_f64) * t68342 + F::cast_from(0.39862222222222222223e1_f64) * t68347 - F::cast_from(0.11958666666666666667e1_f64) * t68350 - F::cast_from(0.71752000000000000002e1_f64) * t68353 - F::cast_from(0.39862222222222222222e0_f64) * t68357 + F::cast_from(0.71752000000000000001e1_f64) * t68360 - F::cast_from(0.47834666666666666668e1_f64) * t68363 + F::cast_from(0.13287407407407407407e1_f64) * t68366 - F::cast_from(0.21908444444444444444e0_f64) * t68368 - F::cast_from(0.48685432098765432099e-1_f64) * t68370 + F::new(0.3071625e0) * t68373 - F::cast_from(0.30428395061728395062e-1_f64) * t43911 - F::cast_from(0.35433086419753086419e0_f64) * t56176 + F::cast_from(0.10629925925925925926e1_f64) * t56183 - F::cast_from(0.79724444444444444444e0_f64) * t56185;
    t68854
}
