//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1166/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1166(t12856: f64, t12963: f64, t12994: f64, t13030: f64, t219: f64, t4488: f64, t10180: f64, t1656: f64, t3366: f64, t1265: f64, t4516: f64, t3365: f64, param_beta: f64) -> (f64, f64, f64, f64, f64) {
    let t13032 = t12856 + t12963 + t12994 + t13030;
    let t13033 = param_beta * t13032;
    let t13035 = t4488 * t219;
    let t13047 = t10180 * t1656 * t3366;
    let t13050 = t4516 * t1265;
    let t13051 = t3365 * t13050;
    (t13032, t13033, t13035, t13047, t13051)
}
