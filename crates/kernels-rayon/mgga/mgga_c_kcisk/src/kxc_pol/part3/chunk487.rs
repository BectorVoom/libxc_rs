//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 487/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk487(t3805: f64, t472: f64, t1333: f64, t1447: f64, t1407: f64, t300: f64, t967: f64, t425: f64, t1350: f64, t443: f64, t1346: f64, t1365: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3806 = t3805 * t472;
    let t3807 = 0.55273148148148148147e-3_f64 * t3806;
    let t3808 = t1333 * t1447;
    let t3810 = t1333 * t1407;
    let t3812 = t967 * t300;
    let t3814 = 0.46853067927761790996e-2_f64 * t3812 * t425;
    let t3815 = t443 * t1350;
    let t3817 = t1346 * t1365;
    (t3806, t3807, t3808, t3810, t3812, t3814, t3815, t3817)
}
