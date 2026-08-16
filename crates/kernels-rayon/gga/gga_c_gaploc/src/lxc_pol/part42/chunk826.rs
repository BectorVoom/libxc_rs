//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 826/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk826(t13258: f64, t484: f64, t11481: f64, t2321: f64, t882: f64, t1063: f64, t11271: f64, t6750: f64, t2268: f64, t3565: f64, t6763: f64, t13310: f64) -> (f64, f64, f64, f64, f64) {
    let t44623 = t484 * t13258;
    let t44624 = 0.15808337019820083111e-2_f64 * t44623;
    let t44626 = t882 * t11481 * t2321;
    let t44627 = 0.11856252764865062333e-2_f64 * t44626;
    let t44630 = 0.85365019907028448797e-1_f64 * t1063 * t11271 * t6750;
    let t44633 = 0.42682509953514224398e0_f64 * t2268 * t3565 * t6763;
    let t44634 = t484 * t13310;
    (t44624, t44627, t44630, t44633, t44634)
}
