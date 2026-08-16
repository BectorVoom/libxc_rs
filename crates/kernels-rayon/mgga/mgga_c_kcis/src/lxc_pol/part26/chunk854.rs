//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 854/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk854(t14955: f64, t5977: f64, t5969: f64, t11670: f64, t538: f64, t2018: f64, t456: f64, t3820: f64, t562: f64, t2029: f64, t318: f64, t86: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17583 = t14955 * t5977;
    let t17586 = 0.5895802469135802469e-1_f64 * t14955 * t5969;
    let t17594 = t11670 * t538;
    let t17613 = t2018 * t456;
    let t17627 = t562 * t3820;
    let t17641 = t86 * t318 * t2029;
    (t17583, t17586, t17594, t17613, t17627, t17641)
}
