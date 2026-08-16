//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 826/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk826(t40805: f64, t4669: f64, t128: f64, t30526: f64, t338: f64, t6444: f64, t39665: f64, t5259: f64, t38569: f64, t7782: f64, t321: f64, t8712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t40806 = t4669 * t40805;
    let t40823 = t30526 * t128;
    let t40826 = t6444 * t338;
    let t40831 = t5259 * t39665;
    let t40891 = t7782 * t38569;
    let t40897 = t8712 * t321;
    (t40806, t40823, t40826, t40831, t40891, t40897)
}
