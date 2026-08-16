//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1067/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1067(t16404: f64, t16471: f64, t1882: f64, t7073: f64, t959: f64, t11775: f64, t28254: f64, t11990: f64, t2817: f64, t11997: f64, t2639: f64, t188: f64, t1903: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33226 = t7073 * t1882 * t16471 * t959 * t16404;
    let t33228 = t11775 * t28254;
    let t33230 = t11990 * t2817;
    let t33232 = t11997 * t2639;
    let t33235 = t188 * t1903 * pi;
    (t33226, t33228, t33230, t33232, t33235)
}
