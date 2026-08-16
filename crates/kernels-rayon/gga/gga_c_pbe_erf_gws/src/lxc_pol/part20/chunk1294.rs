//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1294/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1294(t2409: f64, t35428: f64, t3959: f64, t14791: f64, t353: f64, t3721: f64, t859: f64, t11553: f64, t50998: f64, t53156: f64, t1123: f64, t51021: f64, t56246: f64, t810: f64) -> (f64, f64, f64, f64) {
    let t56483 = t3959 * t2409 * t35428;
    let t56491 = t859 * t353 * t14791 * t3721;
    let t56495 = t50998 * t53156 * t11553;
    let t56500 = t50998 * t51021 * t1123 * t56246 * t810;
    (t56483, t56491, t56495, t56500)
}
