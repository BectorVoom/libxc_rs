//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1272/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1272(t28986: f64, t572: f64, t7002: f64, t1916: f64, t32776: f64, t1936: f64, t2055: f64, t4292: f64, t1518: f64, t7373: f64, t34359: f64, t1459: f64, t34363: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t129055 = 6.0_f64 * t572 * t28986 * t7002;
    let t129057 = 6.0_f64 * t1916 * t32776;
    let t129065 = 6.0_f64 * t572 * t4292 * t2055 * t1936;
    let t129069 = 6.0_f64 * t572 * t1518 * t7373 * t1936;
    let t129072 = 6.0_f64 * t572 * t34359 * t7002;
    let t129078 = 6.0_f64 * t1459 * t34363;
    (t129055, t129057, t129065, t129069, t129072, t129078)
}
