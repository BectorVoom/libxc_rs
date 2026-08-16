//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1054/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1054(t74381: f64, t74387: f64, t74390: f64, t74396: f64, t74403: f64, t74406: f64, t74408: f64, t74414: f64, t74436: f64, t76986: f64, t76998: f64, t76999: f64, t77004: f64, t77005: f64, t77006: f64, t77007: f64) -> f64 {
    let t80098 = -0.39418438709028076168e-5_f64 * t74381 + t76986 + 0.70077224371605468748e-6_f64 * t74387 - 0.70077224371605468748e-6_f64 * t74390 - 0.10511583655740820312e-5_f64 * t74396 + t76998 - t76999 + 0.35038612185802734374e-6_f64 * t74403 - 0.35038612185802734374e-6_f64 * t74406 - 0.58171619854173713844e-5_f64 * t74408 - 0.58171619854173713844e-5_f64 * t74414 + t77004 + t77005 + t77006 + t77007 - 0.35038612185802734374e-6_f64 * t74436;
    t80098
}
