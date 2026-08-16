//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1142/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1142(t11775: f64, t28254: f64, t11990: f64, t2817: f64, t11997: f64, t2639: f64, t188: f64, t1903: f64, t190: f64, t2660: f64, t286: f64, t442: f64, t8139: f64) -> (f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t33228 = t11775 * t28254;
    let t33230 = t11990 * t2817;
    let t33232 = t11997 * t2639;
    let t33235 = t188 * t1903 * pi;
    let t33240 = t2660 * t33235 * t8139 * t190 * t286 * t442;
    (t33228, t33230, t33232, t33235, t33240)
}
