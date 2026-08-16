//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 840/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk840(t2367: f64, t2634: f64, t930: f64, t7406: f64, t914: f64, t2601: f64, t7178: f64, t2270: f64, t2723: f64, t2722: f64, t2274: f64, t2813: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8024 = t2367 * t2634;
    let t8025 = t930 * t8024;
    let t8027 = t914 * t7406;
    let t8036 = t2601 * t7178;
    let t8037 = t914 * t8036;
    let t8040 = t2270 * t2723;
    let t8041 = t2722 * t8040;
    let t8044 = t2723 * t2274;
    let t8045 = t2813 * t8044;
    (t8024, t8025, t8027, t8036, t8037, t8040, t8041, t8044, t8045)
}
