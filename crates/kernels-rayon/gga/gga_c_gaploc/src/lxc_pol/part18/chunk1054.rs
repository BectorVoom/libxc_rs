//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1054/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1054(t21446: f64, t7290: f64, t2530: f64, t701: f64, t2610: f64, t1835: f64, t935: f64, t1878: f64, t481: f64, t941: f64, t325: f64, t7112: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21451 = t7290 * t21446;
    let t21455 = t2530 * t701;
    let t21456 = t2610 * t21455;
    let t21460 = t935 * t1835;
    let t21461 = t2610 * t21460;
    let t21476 = t481 * t941 * t1878;
    let t21483 = t325 * t7112;
    (t21451, t21455, t21456, t21460, t21461, t21476, t21483)
}
