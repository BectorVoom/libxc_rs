//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 920/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk920(t10662: f64, t3270: f64, t105: f64, t494: f64, t97: f64, t1065: f64, t481: f64, t2104: f64, t3436: f64, t2302: f64) -> (f64, f64, f64, f64, f64) {
    let t10663 = t3270 * t10662;
    let t10666 = t105 * t494;
    let t10667 = t97 * t10666;
    let t10668 = t1065 * t481;
    let t10669 = t3270 * t10668;
    let t10672 = t2104 * t3436;
    let t10673 = t2302 * t10672;
    (t10663, t10666, t10667, t10669, t10673)
}
