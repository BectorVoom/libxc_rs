//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 936/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk936(t10304: f64, t1095: f64, t13540: f64, t13571: f64, t801: f64, t13542: f64, t10883: f64, t13538: f64, t13547: f64, t13553: f64, t13556: f64, t13562: f64, t13565: f64, t2380: f64) -> (f64, f64, f64) {
    let t14541 = t10304 * t1095;
    let t14544 = 0.6419148148148148148e-1_f64 * t13540;
    let t14550 = t801 * t13571;
    let t14553 = 0.19257444444444444444e0_f64 * t13542;
    let t14554 = 0.1760655e0_f64 * t14541 * t2380 + t14544 - 0.9628722222222222222e-1_f64 * t13556 - 0.1604787037037037037e0_f64 * t13547 + 0.38514888888888888888e0_f64 * t13553 + 0.28886166666666666666e0_f64 * t13565 - 0.11554466666666666666e1_f64 * t13562 + 0.234754e0_f64 * t14550 - t10883 - 0.6419148148148148148e-1_f64 * t13538 - t14553;
    (t14541, t14550, t14554)
}
