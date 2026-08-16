//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 734/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk734(t201: f64, t4443: f64, t1976: f64, t674: f64, t16156: f64, t7251: f64, t7738: f64, t7376: f64, t7244: f64, t7259: f64, t7541: f64, t7715: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34855 = t201 * t4443;
    let t34857 = t1976 * t34855 * t674;
    let t34869 = t16156 * t7251;
    let t34871 = t16156 * t7738;
    let t34873 = t16156 * t7376;
    let t34875 = t7244 * t7259;
    let t34878 = t7541 * t7715 * t674;
    (t34855, t34857, t34869, t34871, t34873, t34875, t34878)
}
