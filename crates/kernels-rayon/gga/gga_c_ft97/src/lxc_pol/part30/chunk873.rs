//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 873/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk873(t35972: f64, t852: f64, t1486: f64, t193: f64, t10248: f64, t33961: f64, t992: f64, t446: f64, t1212: f64, t33966: f64, t89: f64, t6222: f64, t7021: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t35973 = t852 * t35972;
    let t35975 = t1486 * t193 * t35973;
    let t35978 = t10248 * t33961 * t992;
    let t35979 = t446 * t35978;
    let t35981 = t33966 * t1212;
    let t35982 = t193 * t35981;
    let t35983 = t89 * t35982;
    let t35985 = t6222 * t7021;
    (t35973, t35975, t35978, t35979, t35981, t35983, t35985)
}
