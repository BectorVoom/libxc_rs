//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 787/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk787(t27: f64, t8420: f64, t16058: f64, t69609: f64, t8425: f64, t16064: f64, t15061: f64, t7487: f64, t34975: f64, t34976: f64, t665: f64, t8455: f64) -> (f64, f64, f64, f64) {
    let t74211 = t27 * t8420;
    let t74213 = t69609 * t16058 * t74211;
    let t74215 = t27 * t8425;
    let t74217 = t69609 * t16064 * t74215;
    let t74219 = t7487 * t15061;
    let t74225 = 0.1064114997332445985e-4_f64 * t34975 * t34976 * t665 * t8455;
    (t74213, t74217, t74219, t74225)
}
