//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 846/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk846(t1341: f64, t575: f64, t638: f64, t7310: f64, t7244: f64, t8427: f64, t2001: f64, t326: f64, t498: f64, t559: f64, t40948: f64, t903: f64) -> (f64, f64, f64, f64) {
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    let t42054 = t2001 * t326 * t559 * t498;
    let t42057 = t903 * t40948;
    (t42042, t42044, t42054, t42057)
}
