//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1097/1293 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1097(t10769: f64, t39409: f64, t2547: f64, t37764: f64, t25397: f64, t37945: f64, t38031: f64, t10710: f64, t10768: f64, t25737: f64, t25499: f64, t37586: f64) -> (f64, f64, f64, f64, f64) {
    let t39410 = t39409 * t10769;
    let t39420 = t37764 * t2547;
    let t39429 = t38031 * t37945 * t25397;
    let t39437 = t10768 * t10710 * t25737;
    let t39440 = t37586 * t10710 * t25499;
    (t39410, t39420, t39429, t39437, t39440)
}
