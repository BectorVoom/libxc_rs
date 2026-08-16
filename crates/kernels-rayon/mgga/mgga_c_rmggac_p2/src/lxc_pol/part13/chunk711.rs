//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 711/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk711(t117: f64, t4685: f64, t4968: f64, t5011: f64, t2000: f64, t326: f64, t1985: f64, t797: f64, t838: f64, t1343: f64, t2048: f64, t29: f64, t3899: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10792 = t4685 * t117;
    let t10820 = t4968 * t117;
    let t11905 = t5011 * t117;
    let t14237 = t2000 * t326;
    let t14243 = t1985 * t797;
    let t14249 = t1985 * t838;
    let t14267 = t2048 * t1343;
    let t14366 = t3899 * t29;
    (t10792, t10820, t11905, t14237, t14243, t14249, t14267, t14366)
}
