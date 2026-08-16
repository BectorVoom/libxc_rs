//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 494/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk494(t3906: f64, t416: f64, t467: f64, t471: f64, t415: f64, t392: f64, t494: f64) -> (f64, f64, f64, f64) {
    let t3907 = t416 * t3906;
    let t3908 = t3907 * t467;
    let t3909 = t3908 * t471;
    let t3910 = t415 * t3909;
    let t3913 = 1.0_f64 / t392 / t494;
    (t3908, t3909, t3910, t3913)
}
