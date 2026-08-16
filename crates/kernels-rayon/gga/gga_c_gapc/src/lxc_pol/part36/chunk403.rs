//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 403/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk403(t1971: f64, t204: f64, t1645: f64, t676: f64, t618: f64, t623: f64, t617: f64, t1403: f64, t203: f64, t153: f64, t181: f64, t628: f64, t655: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t1972 = t1971 * t204;
    let t1975 = t1645 * pi;
    let t1976 = t1975 * t676;
    let t1979 = t618 * t623;
    let t1980 = t617 * t1979;
    let t1983 = t203 * t1403;
    let t1984 = t153 * t1983;
    let t1985 = t181 * t1984;
    let t1988 = t628 * t655;
    (t1972, t1975, t1976, t1979, t1980, t1983, t1985, t1988)
}
