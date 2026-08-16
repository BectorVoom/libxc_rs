//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 458/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk458(t1005: f64, t383: f64, t126: f64, t3056: f64, t1631: f64, t2014: f64, t2021: f64, t3359: f64, t3360: f64, t3364: f64, t3368: f64, t534: f64) -> f64 {
    let t3371 = t1005 * t383;
    let t3374 = t3056 * t126;
    let t3379 = -0.11705142615505742e0_f64 * t3359 * t3360 + 0.23410285231011484e0_f64 * t3364 - 0.26564305359272358183e-2_f64 * t2014 * t3368 + 0.319782988780431561e-1_f64 * t2021 * t3371 - 0.532971647967385935e-1_f64 * t534 * t3374 + 0.13977476158628290272e-1_f64 * t1631 * t3371;
    t3379
}
