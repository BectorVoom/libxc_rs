//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1066/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1066(t2144: f64, t3351: f64, t352: f64, t7231: f64, t8502: f64, t2001: f64, t326: f64, t498: f64, t559: f64, t7720: f64, t40948: f64, t903: f64) -> (f64, f64, f64) {
    let t42050 = t3351 * t7231 * t2144 * t8502 * t352;
    let t42054 = t2001 * t326 * t559 * t498;
    let t42055 = t7720 * t42054;
    let t42057 = t903 * t40948;
    (t42050, t42055, t42057)
}
