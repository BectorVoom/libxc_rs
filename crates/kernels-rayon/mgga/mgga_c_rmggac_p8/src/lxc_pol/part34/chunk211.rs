//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 211/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk211(t1433: f64, t1438: f64, t453: f64, t592: f64, t1152: f64, t1157: f64, t1392: f64, t1430: f64, t198: f64, t446: f64, t454: f64, t589: f64) -> (f64, f64) {
    let t1439 = t1433 + t1438;
    let t1442 = t592 * t453;
    let t1451 = -0.32163648644302209643e2_f64 * t1439 * t198 + 0.96490945932906628929e2_f64 * t1442 * t446 + 0.96490945932906628929e2_f64 * t1152 * t589 - 0.38596378373162651572e3_f64 * t1157 * t1430 + 0.96490945932906628929e2_f64 * t454 * t1392;
    (t1439, t1451)
}
