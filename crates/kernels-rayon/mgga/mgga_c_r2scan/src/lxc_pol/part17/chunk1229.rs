//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1229/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1229(t38568: f64, t39846: f64, t41576: f64, t41577: f64, t41578: f64, t41584: f64, t43266: f64, t43269: f64, t43271: f64, t43273: f64, t43275: f64, t43277: f64) -> f64 {
    let t44316 = t41576 - t41577 + 0.54878743191129263322e-2_f64 * t43266 - t38568 - 0.13099107994629972538e-1_f64 * t43269 - 0.26198215989259945076e-1_f64 * t43271 - 0.1047928639570397803e0_f64 * t43273 + 0.43663693315433241794e-2_f64 * t43275 + t41578 + 0.10975748638225852664e-1_f64 * t43277 - 0.16951189180550569635e1_f64 * t39846 - t41584;
    t44316
}
