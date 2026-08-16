//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 748/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk748(t72019: f64, t797: f64, t333: f64, t4669: f64, t71949: f64, t305: f64, t71637: f64, t14506: f64, t2085: f64, t69518: f64, t69521: f64, t14584: f64, t504: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t72023 = t797 * t72019;
    let t72027 = t4669 * t71949 * t333;
    let t72037 = t305 * t71637;
    let t72038 = 0.14635184302277988245e0_f64 * t72037;
    let t72062 = t14506 * t2085;
    let t72087 = 0.34547904762044099522e0_f64 * t69518;
    let t72088 = 0.50557909407869413937e0_f64 * t69521;
    let t72109 = t504 * t14584;
    (t72023, t72027, t72038, t72062, t72087, t72088, t72109)
}
