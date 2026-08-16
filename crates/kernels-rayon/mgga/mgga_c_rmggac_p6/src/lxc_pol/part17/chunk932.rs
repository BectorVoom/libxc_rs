//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 932/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk932(t8616: f64, t9222: f64, t118: f64, t128: f64, t1451: f64, t1986: f64, t8571: f64, t10043: f64, t674: f64, t7715: f64, t1997: f64, t10084: f64, t16043: f64) -> (f64, f64, f64, f64) {
    let t45514 = t9222 * t8616;
    let t45519 = t8571 * t1986 * t118 * t128 * t1451;
    let t45522 = t10043 * t7715 * t674;
    let t45523 = t45522 * t1997;
    let t45525 = t16043 * t10084;
    (t45514, t45519, t45523, t45525)
}
