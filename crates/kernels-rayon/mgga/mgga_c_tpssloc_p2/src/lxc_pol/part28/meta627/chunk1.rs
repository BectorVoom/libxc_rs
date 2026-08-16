//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1956/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1956(t26055: f64, t7032: f64, t22531: f64, t22537: f64, t23963: f64, t26911: f64, t6492: f64, t7782: f64, t90196: f64, t91890: f64, t91894: f64, t91896: f64, t91898: f64, t91900: f64, t91904: f64, t91905: f64, t91907: f64) -> f64 {
    let t91913 = 32.0_f64 / 9.0_f64 * t26055 * t7032;
    let t91914 = t91890 - 2.0_f64 / 3.0_f64 * t22537 * t7782 + t91894 + t91896 + t91898 + t91900 + 10.0_f64 * t23963 * t90196 + t91904 - 176.0_f64 / 27.0_f64 * t91905 - 10.0_f64 / 3.0_f64 * t91907 * t6492 - 5.0_f64 / 3.0_f64 * t26911 * t22531 + t91913;
    t91914
}
