//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 706/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk706(t9075: f64, t9079: f64, t9083: f64, t9231: f64, t117: f64, t4685: f64, t4968: f64, t5011: f64, t2000: f64, t326: f64, t1985: f64, t797: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10062 = 0.79828278012425390426e-1_f64 * t9075;
    let t10081 = 0.79828278012425390426e-1_f64 * t9079;
    let t10109 = 0.72042316457491791906e-3_f64 * t9083;
    let t10202 = 2.0_f64 * t9231;
    let t10792 = t4685 * t117;
    let t10820 = t4968 * t117;
    let t11905 = t5011 * t117;
    let t14237 = t2000 * t326;
    let t14243 = t1985 * t797;
    (t10062, t10081, t10109, t10202, t10792, t10820, t11905, t14237, t14243)
}
