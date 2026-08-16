//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1134/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1134(t3009: f64, t5199: f64, t2973: f64, t5177: f64, t1082: f64, t1089: f64, t12244: f64, t4068: f64, t11976: f64, t4109: f64, t1042: f64, t5082: f64) -> (f64, f64, f64, f64, f64) {
    let t15601 = 0.17315859105681463759e2_f64 * t3009 * t5199;
    let t15602 = t2973 * t5177;
    let t15603 = t15602 * t1082;
    let t15605 = 0.11696447245269292414e1_f64 * t1089 * t15603;
    let t15607 = 4.0_f64 * t12244 * t4068;
    let t15609 = 0.32163958997385070134e2_f64 * t11976 * t4109;
    let t15610 = t5082 * t1042;
    (t15601, t15605, t15607, t15609, t15610)
}
