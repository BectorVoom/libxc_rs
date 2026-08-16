//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 713/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk713(t69157: f64, t7204: f64, t69161: f64, t7192: f64, t140: f64, t212: f64, t3151: f64, t4071: f64, t672: f64, t1330: f64, t236: f64, t899: f64) -> (f64, f64, f64, f64) {
    let t69976 = t7204 * t69157;
    let t69983 = t7192 * t69161;
    let t69995 = t672 * t212 * t4071 * t140 * t3151;
    let t70018 = t899 * t236 * t1330;
    (t69976, t69983, t69995, t70018)
}
