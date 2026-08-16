//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1152/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1152(t38145: f64, t6085: f64, t7922: f64, t6093: f64, t7605: f64, t8081: f64, t7619: f64, t2147: f64, t7624: f64, t10719: f64, t8198: f64, t1575: f64, t269: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40041 = t6085 * t38145 * t7922;
    let t40044 = t6093 * t38145 * t7605;
    let t40047 = t6085 * t38145 * t8081;
    let t40050 = t6093 * t38145 * t7619;
    let t40053 = t2147 * t38145 * t7624;
    let t40059 = t8198 * t10719;
    let t40061 = t1575 * t269;
    (t40041, t40044, t40047, t40050, t40053, t40059, t40061)
}
