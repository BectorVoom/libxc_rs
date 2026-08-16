//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 821/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk821(t16036: f64, t550: f64, t6976: f64, t1992: f64, t16040: f64, t7696: f64, t794: f64, t6897: f64, t12461: f64, t2094: f64, t26163: f64, t193: f64, t200: f64, t2056: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26461 = t16036 * t550;
    let t26462 = t6976 * t26461;
    let t26463 = t1992 * t26462;
    let t26466 = t16040 * t550;
    let t26467 = t6976 * t26466;
    let t26468 = t1992 * t26467;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26558 = t2094 * t12461;
    let t26559 = t26558 * t26163;
    let t26563 = t193 * t200 * t2056;
    (t26463, t26468, t26475, t26558, t26559, t26563)
}
