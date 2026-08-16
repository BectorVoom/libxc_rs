//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1264/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1264(t40005: f64, t4819: f64, t16858: f64, t3657: f64, t56676: f64, t56681: f64, t56686: f64, t56693: f64, t56939: f64, t56941: f64, t56945: f64, t56948: f64, t56950: f64) -> (f64, f64, f64) {
    let t56952 = 0.96490945932906628932e2_f64 * t40005 * t4819;
    let t56954 = 4.0_f64 * t3657 * t16858;
    let t56955 = -t56676 + t56681 + t56686 - t56693 + t56939 + t56941 - t56945 - t56948 + t56950 + t56952 + t56954;
    (t56952, t56954, t56955)
}
