//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 403/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk403(t1576: f64, t1954: f64, t178: f64, t1936: f64, t567: f64, t647: f64, t1939: f64, t424: f64, t668: f64, t136: f64, t5: f64, t1033: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1955 = t1576 * t1954;
    let t1958 = t178 * t1936;
    let t1959 = t647 * t567;
    let t1960 = t1939 * t1959;
    let t1965 = t424 * t668;
    let t1968 = t136 * t5;
    let t1969 = t1968 * t1033;
    (t1955, t1958, t1960, t1965, t1968, t1969)
}
