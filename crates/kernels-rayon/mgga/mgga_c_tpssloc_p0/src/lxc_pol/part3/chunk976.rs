//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 976/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk976(t1406: f64, t2239: f64, t1437: f64, t2241: f64, t4021: f64, t645: f64, t2307: f64, t1409: f64, t9321: f64, t2291: f64, t3966: f64, t584: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12571 = t1406 * t2239;
    let t12582 = t1437 * t2241;
    let t12585 = t4021 * t645;
    let t12588 = t1437 * t2307;
    let t12595 = t9321 * t1409;
    let t12598 = t2291 * t3966;
    let t12603 = 2.0_f64 * t584;
    (t12571, t12582, t12585, t12588, t12595, t12598, t12603)
}
