//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 841/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk841(t10053: f64, t3330: f64, t818: f64, t959: f64, t3329: f64, t3325: f64, t134: f64, t2404: f64, t3412: f64, t3405: f64, t3411: f64, t2315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let pi = (M_PI as f64);
    let t10054 = t10053 * t3330;
    let t10057 = pi * t818 * t959;
    let t10058 = t10057 * t3329;
    let t10059 = t3325 * t10058;
    let t10061 = t134 * t2404;
    let t10062 = t3412 * t10061;
    let t10063 = t3405 * t10062;
    let t10064 = t3411 * t10063;
    let t10067 = t134 * t2315;
    (t10054, t10057, t10058, t10059, t10063, t10064, t10067)
}
