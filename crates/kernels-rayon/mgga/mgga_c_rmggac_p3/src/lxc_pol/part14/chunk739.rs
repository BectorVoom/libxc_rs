//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 739/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk739(t1327: f64, t356: f64, t640: f64, t7323: f64, t507: f64, t8619: f64, t22: f64, t235: f64, t29837: f64, t1249: f64, t2144: f64, t7900: f64, t892: f64) -> (f64, f64, f64, f64, f64) {
    let t34931 = t7323 * t640 * t356 * t1327;
    let t34938 = t507 * t8619;
    let t34944 = t235 * t29837 * t22;
    let t34957 = t1249 * t2144;
    let t34960 = t892 * t7900;
    (t34931, t34938, t34944, t34957, t34960)
}
