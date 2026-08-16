//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2113/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2113(t10319: f64, t699: f64, t10313: f64, t41654: f64, t270: f64, t276: f64, t39267: f64, t273: f64, t242: f64, t281: f64, t283: f64, t2853: f64, t2860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41887 = t699 * t10319;
    let t41889 = t699 * t10313;
    let t41904 = 280.0_f64 / 81.0_f64 * t41654;
    let t41935 = 1.0_f64 / t276 / t39267 / t270 / 96.0_f64;
    let t41942 = f64::powf(t273, -0.25e1_f64);
    let t41959 = 0.31310740740740740741e1_f64 * t41654;
    let t41961 = t281 * t242 * t283;
    let t41962 = 0.13490888888888888889e1_f64 * t41961;
    let t41981 = t2853 * t2860;
    (t41887, t41889, t41904, t41935, t41942, t41959, t41961, t41962, t41981)
}
