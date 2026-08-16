//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 795/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk795(t12276: f64, t5006: f64, t10399: f64, t5497: f64, t1775: f64, t5507: f64, t695: f64, t1060: f64, t5509: f64, t10777: f64, t41: f64, t10436: f64, t7568: f64) -> (f64, f64, f64, f64, f64) {
    let t12277 = t5006 * t12276;
    let t12280 = t5497 * t10399;
    let t12281 = t1775 * t12280;
    let t12284 = t5507 * t695;
    let t12285 = t1060 * t5509;
    let t12286 = t12284 * t12285;
    let t12287 = t1775 * t12286;
    let t12290 = t10777 * t41;
    let t12306 = t7568 * t10436;
    (t12277, t12281, t12287, t12290, t12306)
}
