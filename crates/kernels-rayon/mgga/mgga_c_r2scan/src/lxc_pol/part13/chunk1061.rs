//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1061/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1061(t37568: f64, t10966: f64, t1103: f64, t269: f64, t607: f64, t10707: f64, t2195: f64, t10710: f64, t20853: f64, t2183: f64, t20590: f64, t20544: f64, t252: f64, t277: f64, t6077: f64, t6261: f64, t783: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t37569 = 0.15243824895787514157e-3_f64 * t37568;
    let t37580 = t10966 * t1103 * t607 * t269;
    let t37582 = t2195 * t10707;
    let t37584 = t37582 * t10710 * t20853;
    let t37586 = t2183 * t10707;
    let t37588 = t37586 * t10710 * t20590;
    let t37599 = t783 * t252 * t20544 / t6077 / t6261 * t277;
    (t37569, t37580, t37582, t37584, t37586, t37588, t37599)
}
