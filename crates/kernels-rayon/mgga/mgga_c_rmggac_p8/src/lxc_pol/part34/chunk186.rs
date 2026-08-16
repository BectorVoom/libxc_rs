//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 186/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk186(t388: f64, t421: f64, t155: f64, t385: f64, t389: f64, t409: f64, t179: f64, t978: f64, t431: f64, t171: f64, t433: f64, t151: f64, t5: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1028 = t388 * t421;
    let t1029 = t155 * t1028;
    let t1031 = t385 * t389;
    let t1037 = t409 * t409;
    let t1038 = 1.0_f64 / t1037;
    let t1040 = t179 * t179;
    let t1041 = 1.0_f64 / t1040;
    let t1042 = t1038 * t978 * t1041;
    let t1044 = 0.17315859105681463759e2_f64 * t431 * t1042;
    let t1045 = t388 * t171;
    let t1046 = t1045 * t433;
    let t1050 = 0.14764627977777777777e-2_f64 * t5 * t959 * t151;
    (t1029, t1031, t1038, t1041, t1044, t1046, t1050)
}
