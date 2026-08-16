//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 896/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk896(t1966: f64, t39850: f64, t34976: f64, t352: f64, t38422: f64, t4550: f64, t1180: f64, t34759: f64, t7472: f64, t8417: f64, t7255: f64, t8432: f64) -> (f64, f64, f64, f64) {
    let t39851 = t1966 * t39850;
    let t39855 = t39851 * t34976 * t38422 * t4550 * t352;
    let t39857 = t1180 * t34759;
    let t39859 = t7472 * t39857 * t8417;
    let t39861 = t7255 * t8432;
    (t39851, t39855, t39859, t39861)
}
