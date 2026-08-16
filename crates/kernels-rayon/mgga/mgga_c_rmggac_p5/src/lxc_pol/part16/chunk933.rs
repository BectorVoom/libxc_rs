//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 933/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk933(t2320: f64, t38374: f64, t8616: f64, t9222: f64, t118: f64, t128: f64, t1451: f64, t1986: f64, t8571: f64, t10043: f64, t674: f64, t7715: f64) -> (f64, f64, f64, f64) {
    let t45509 = t38374 * t2320;
    let t45514 = t9222 * t8616;
    let t45519 = t8571 * t1986 * t118 * t128 * t1451;
    let t45522 = t10043 * t7715 * t674;
    (t45509, t45514, t45519, t45522)
}
