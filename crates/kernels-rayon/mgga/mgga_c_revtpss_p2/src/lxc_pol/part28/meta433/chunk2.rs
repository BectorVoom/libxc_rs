//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1630/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1630(t1678: f64, t3151: f64, t3304: f64, t3302: f64, t4893: f64, t15609: f64, t15604: f64, t1089: f64, t1668: f64, t3259: f64, t15780: f64, t4983: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16426 = t1678 * t3151;
    let t16427 = t16426 * t3304;
    let t16432 = t4893 * t3302;
    let t16433 = t16432 * t15609;
    let t16436 = t16432 * t15604;
    let t16440 = t3259 * t1668 * t1089;
    let t16443 = t15780 * t4983;
    (t16426, t16427, t16433, t16436, t16440, t16443)
}
