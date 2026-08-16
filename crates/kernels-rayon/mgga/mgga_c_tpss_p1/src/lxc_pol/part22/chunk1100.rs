//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1100/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1100(t1062: f64, t12066: f64, t1072: f64, t4155: f64, t1535: f64, t2998: f64, t1054: f64, t1082: f64, t11970: f64, t11973: f64, t11975: f64, t11978: f64, t11980: f64, t11982: f64, t1531: f64, t2925: f64, t2994: f64, t3002: f64, t4143: f64, t4158: f64, t9414: f64) -> f64 {
    let t12067 = t12066 * t1062;
    let t12070 = t4155 * t1072;
    let t12075 = t1535 * t2998;
    let t12078 = -t11970 - t11973 - t11975 - t11978 - t11980 - t11982 + 1.0_f64 * t9414 * t1531 + 2.0_f64 * t2925 * t4143 + 1.0_f64 * t1054 * t12067 + 0.11696447245269292414e1_f64 * t12070 * t1082 + 0.5848223622634646207e0_f64 * t4158 * t2994 + 0.17315859105681463759e2_f64 * t12075 * t3002;
    t12078
}
