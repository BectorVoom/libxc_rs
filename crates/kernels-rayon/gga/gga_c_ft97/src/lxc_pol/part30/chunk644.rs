//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 644/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk644(t14163: f64, t27753: f64, t3880: f64, t6135: f64, t10007: f64, t1882: f64, t6863: f64, t6854: f64, t1449: f64, t2360: f64, t3886: f64, t14182: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t28375 = t14163 * t27753;
    let t28378 = t6135 * t3880;
    let t28379 = t10007 * t28378;
    let t28382 = t1882 * t6863;
    let t28384 = t1882 * t6854;
    let t28386 = t1449 * t2360;
    let t28387 = t28386 * t3886;
    let t28388 = t14182 * t28387;
    (t28375, t28378, t28379, t28382, t28384, t28387, t28388)
}
