//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1422/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1422(t241: f64, t59258: f64, t59367: f64, t59379: f64, t59428: f64, t59191: f64, t59193: f64, t59196: f64, t59199: f64, t59202: f64, t59205: f64, t59209: f64, t59212: f64, t59214: f64, t59218: f64, t59220: f64) -> (f64, f64) {
    let t59431 = t241 * (t59258 + t59367 + t59379 + t59428);
    let t59432 = -t59191 + t59193 - t59196 - t59199 + t59202 + t59205 + t59209 + t59212 - t59214 - t59218 - t59220 + t59431;
    (t59431, t59432)
}
