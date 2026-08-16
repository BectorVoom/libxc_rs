//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 560/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk560(t1882: f64, t5646: f64, t5719: f64, t8392: f64, t103: f64, t5617: f64, t1332: f64, t1851: f64, t5733: f64, t358: f64, t5728: f64, t487: f64, t5743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23229 = t1882 * t5646;
    let t23239 = t8392 * t5719;
    let t23244 = t103 * t5617;
    let t23249 = t1851 * t1332;
    let t23263 = t1882 * t5733;
    let t23265 = t1332 * t358;
    let t23283 = t1882 * t5728;
    let t23294 = t487 * t5743;
    (t23229, t23239, t23244, t23249, t23263, t23265, t23283, t23294)
}
