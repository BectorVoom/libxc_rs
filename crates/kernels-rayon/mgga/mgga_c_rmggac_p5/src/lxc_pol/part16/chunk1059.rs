//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1059/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1059(t236: f64, t615: f64, t1981: f64, t41799: f64, t676: f64, t46832: f64, t7473: f64, t7478: f64, t40702: f64, t8571: f64, t40081: f64, t46434: f64, t7198: f64) -> (f64, f64, f64, f64, f64) {
    let t48033 = t236 * t615;
    let t48036 = t41799 * t1981 * t676 * t48033;
    let t48038 = t46832 * t7473;
    let t48039 = t48038 * t7478;
    let t48041 = t8571 * t40702;
    let t48043 = t8571 * t40081;
    let t48047 = t7198 * t46434;
    (t48036, t48039, t48041, t48043, t48047)
}
