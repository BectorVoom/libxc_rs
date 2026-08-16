//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 441/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk441(t2109: f64, t322: f64, t178: f64, t1832: f64, t108: f64, t670: f64, t14: f64, t260: f64, t435: f64, t341: f64, t19: f64, t271: f64) -> (f64, f64, f64, f64, f64) {
    let t2110 = t2109 * t322;
    let t2113 = t1832 * t178;
    let t2116 = t670 * t108;
    let t2117 = t2116 * t14;
    let t2122 = t260 * t435;
    let t2123 = t2122 * t341;
    let t2124 = t271 * t19;
    (t2110, t2113, t2117, t2123, t2124)
}
