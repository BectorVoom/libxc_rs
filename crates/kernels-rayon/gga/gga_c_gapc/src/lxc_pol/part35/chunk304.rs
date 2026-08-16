//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 304/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk304(t27: f64, t345: f64, t13: f64, t362: f64, t363: f64) -> (f64, f64) {
    let t1185 = t345 * t27;
    let t1186 = 1.0_f64 / t1185;
    let t1187 = t13 * t1186;
    let t1188 = t362 * t362;
    let t1189 = t1188 * t363;
    let t1191 = 2.0_f64 * t1187 * t1189;
    (t1188, t1191)
}
