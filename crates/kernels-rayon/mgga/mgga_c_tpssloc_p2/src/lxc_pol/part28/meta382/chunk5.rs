//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1465/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1465(t3359: f64, t4819: f64, t1136: f64, t3351: f64, t4823: f64, t11352: f64, t1682: f64, t3333: f64, t1155: f64, t4858: f64, t1695: f64, t3395: f64) -> (f64, f64, f64, f64, f64) {
    let t15164 = t4819 * t3359;
    let t15165 = t15164 * t1136;
    let t15168 = t4823 * t3351;
    let t15171 = t1682 * t11352;
    let t15172 = t15171 * t3333;
    let t15179 = t4858 * t1155;
    let t15182 = t1695 * t3395;
    (t15165, t15168, t15172, t15179, t15182)
}
