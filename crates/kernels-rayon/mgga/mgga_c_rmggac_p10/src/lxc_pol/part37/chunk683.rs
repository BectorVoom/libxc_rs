//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 683/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk683(t465: f64, t68759: f64, t7472: f64, t14229: f64, t34846: f64, t270: f64, t668: f64, t31: f64, t7349: f64, t7351: f64, t2019: f64, t3061: f64, t7926: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t68760 = t465 * t68759;
    let t68761 = t7472 * t68760;
    let t68764 = t34846 * t14229;
    let t68788 = t668 * t270;
    let t68791 = t7349 * t7351 * t68788 * t31;
    let t68794 = t2019 * t7926 * t3061;
    (t68760, t68761, t68764, t68788, t68791, t68794)
}
