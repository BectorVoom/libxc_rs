//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 492/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk492(t205: f64, t6017: f64, t23: f64, t600: f64, t1839: f64, t4388: f64, t446: f64, t1392: f64, t1487: f64, t1156: f64, t1835: f64, t472: f64, t6067: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6218 = t6017 * t205;
    let t6224 = t600 * t23;
    let t6231 = t4388 * t1839;
    let t6232 = t6231 * t446;
    let t6235 = t1487 * t1392;
    let t6240 = t1156 * t1835;
    let t6241 = t6240 * t446;
    let t6244 = t472 * t6067;
    (t6218, t6224, t6232, t6235, t6241, t6244)
}
