//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 832/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk832(t3725: f64, t7819: f64, t1203: f64, t7796: f64, t1556: f64, t8396: f64, t8307: f64, t1308: f64, t3973: f64, t8327: f64, t1580: f64, t8323: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t27613 = t7819 * t3725;
    let t27627 = t7796 * t1203;
    let t27694 = t8396 * t1556;
    let t27705 = t8307 * sigma0;
    let t27706 = t27705 * t1308;
    let t27709 = t3973 * t8327;
    let t27710 = t1580 * t27709;
    let t27777 = t3973 * t8323;
    (t27613, t27627, t27694, t27706, t27710, t27777)
}
