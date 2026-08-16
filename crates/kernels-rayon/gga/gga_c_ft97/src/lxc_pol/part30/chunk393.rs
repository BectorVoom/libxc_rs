//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 393/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk393(t6353: f64, t875: f64, t296: f64, t1503: f64, t1882: f64, t1501: f64, t870: f64, t684: f64, t2881: f64, t824: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6354 = t6353 * t875;
    let t6355 = t296 * t6354;
    let t6359 = t1882 * t1503 / 9.0_f64;
    let t6360 = t870 * t1501;
    let t6361 = t6360 * t684;
    let t6362 = t2881 * t6361;
    let t6365 = t1501 * t824;
    (t6355, t6359, t6360, t6361, t6362, t6365)
}
