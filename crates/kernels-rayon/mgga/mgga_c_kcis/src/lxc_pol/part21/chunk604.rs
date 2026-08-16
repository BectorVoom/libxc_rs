//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 604/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk604(t1022: f64, t4772: f64, t1096: f64, t1092: f64, t1768: f64, t3178: f64, t1709: f64, t2811: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4773 = t1022 * t4772;
    let t4774 = t1096 * t4773;
    let t4775 = t1092 * t4774;
    let t4778 = t3178 * t1768;
    let t4779 = t1092 * t4778;
    let t4781 = t1709 * t2811;
    (t4773, t4774, t4775, t4778, t4779, t4781)
}
