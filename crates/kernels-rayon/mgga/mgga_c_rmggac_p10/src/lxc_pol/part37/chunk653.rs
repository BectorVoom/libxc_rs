//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 653/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk653(t2185: f64, t7472: f64, t16155: f64, t7229: f64, t507: f64, t8619: f64, t22: f64, t235: f64, t29837: f64, t16502: f64, t118: f64, t1985: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34881 = t7472 * t2185;
    let t34884 = t7229 * t16155;
    let t34938 = t507 * t8619;
    let t34944 = t235 * t29837 * t22;
    let t34975 = t7229 * t16502;
    let t34976 = t1985 * t118;
    (t34881, t34884, t34938, t34944, t34975, t34976)
}
