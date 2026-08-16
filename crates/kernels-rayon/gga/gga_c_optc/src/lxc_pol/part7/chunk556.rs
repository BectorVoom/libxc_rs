//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 556/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk556(t2722: f64, t2724: f64, t2620: f64, t331: f64, t2246: f64, t329: f64, t155: f64, t889: f64, t947: f64, t146: f64, t2341: f64, t318: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2725 = t2722 * t2724;
    let t2729 = 0.16793568152788065763e-2_f64 * t331 * t2620;
    let t2730 = t329 * t2246;
    let t2731 = t155 * t2730;
    let t2734 = t947 * t889;
    let t2737 = t146 * t318 * t2341;
    (t2725, t2729, t2730, t2731, t2734, t2737)
}
