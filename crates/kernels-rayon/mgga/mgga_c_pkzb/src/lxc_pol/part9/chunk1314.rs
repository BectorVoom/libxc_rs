//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1314/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1314(t2368: f64, t824: f64, t300: f64, t3175: f64, t3185: f64, t8381: f64, t926: f64, t8423: f64, t8428: f64, t8431: f64, t54: f64, t8253: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23167 = t2368 * t824;
    let t23176 = t300 * t3175;
    let t23201 = t3185 * t926 * t8381;
    let t23204 = t3185 * t926 * t8423;
    let t23207 = t8428 * t926 * t8431;
    let t23213 = t54 * t8253;
    (t23167, t23176, t23201, t23204, t23207, t23213)
}
