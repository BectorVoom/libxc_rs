//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 745/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk745(t1291: f64, t2039: f64, t270: f64, t638: f64, t2046: f64, t2050: f64, t31: f64, t1277: f64, t2085: f64, t7315: f64, t5016: f64, t7707: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35114 = t638 * t2039 * t1291 * t270;
    let t35118 = t2046 * t2050 * t1291 * t31;
    let t35124 = t638 * t2039 * t1277 * t270;
    let t35128 = t2046 * t2050 * t1277 * t31;
    let t35130 = t7315 * t2085;
    let t35132 = t5016 * t7707;
    (t35114, t35118, t35124, t35128, t35130, t35132)
}
