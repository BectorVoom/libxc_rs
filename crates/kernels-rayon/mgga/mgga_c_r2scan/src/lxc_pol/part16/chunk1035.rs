//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1035/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1035(t146: f64, t6091: f64, t978: f64, t2145: f64, t2832: f64, t537: f64, t8691: f64, t277: f64, t3216: f64, t6212: f64, t3016: f64, t8001: f64, t910: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t26282 = t146 * t6091 * t978;
    let t27067 = t146 * t2145 * t2832;
    let t27661 = t537 * t8691;
    let t27914 = t277 * t8691;
    let t27955 = t6212 * t3216;
    let t27977 = t6212 * t3016;
    let t27996 = t8001 * t910;
    (t26282, t27067, t27661, t27914, t27955, t27977, t27996)
}
