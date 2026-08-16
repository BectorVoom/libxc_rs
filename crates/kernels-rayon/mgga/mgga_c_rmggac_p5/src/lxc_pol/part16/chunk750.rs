//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 750/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk750(t2185: f64, t7716: f64, t1004: f64, t107: f64, t490: f64, t1180: f64, t673: f64, t7472: f64, t7487: f64, t7757: f64, t1326: f64, t1330: f64) -> (f64, f64, f64, f64, f64) {
    let t35151 = t7716 * t2185;
    let t35154 = t1004 * t107;
    let t35155 = t490 * t35154;
    let t35190 = t1180 * t673;
    let t35191 = t7472 * t35190;
    let t35204 = t7487 * t7757;
    let t35206 = t1326 * t1330;
    (t35151, t35155, t35191, t35204, t35206)
}
