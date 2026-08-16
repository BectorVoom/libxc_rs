//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 503/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk503(t191: f64, t7640: f64, t815: f64, t287: f64, t2404: f64, t798: f64, t2770: f64, t863: f64, t848: f64, t2344: f64, t2680: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10261 = t191 * t7640;
    let t10362 = t815 * t815;
    let t10363 = 1.0_f64 / t10362;
    let t10364 = t287 * t10363;
    let t10409 = t2404 * t798;
    let t10443 = t2770 * t863;
    let t10447 = t848 * t863;
    let t10478 = t2344 * t798;
    let t10491 = t665 * t2680;
    (t10261, t10362, t10363, t10364, t10409, t10443, t10447, t10478, t10491)
}
