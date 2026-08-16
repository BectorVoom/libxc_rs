//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1029/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1029(t1227: f64, t5728: f64, t2368: f64, t6517: f64, t406: f64, t6524: f64, t8427: f64) -> (f64, f64, f64, f64, f64) {
    let t8429 = t1227 * t5728;
    let t8430 = t6517 * t2368;
    let t8431 = t8429 * t8430;
    let t8432 = t406 * t8431;
    let t8435 = t6524 * t8427;
    (t8429, t8430, t8431, t8432, t8435)
}
