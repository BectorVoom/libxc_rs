//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 667/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk667(t2269: f64, t362: f64, t2263: f64, t1093: f64, t3061: f64, t1218: f64, t491: f64, t1217: f64) -> (f64, f64, f64, f64, f64) {
    let t4039 = t362 * t2269;
    let t4044 = t362 * t2263;
    let t4219 = t3061 * t1093;
    let t4280 = t1218 * t491;
    let t4281 = t1217 * t4280;
    (t4039, t4044, t4219, t4280, t4281)
}
