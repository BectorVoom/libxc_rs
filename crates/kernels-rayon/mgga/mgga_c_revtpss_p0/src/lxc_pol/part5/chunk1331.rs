//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1331/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1331(t1260: f64, t6601: f64, t1222: f64, t1266: f64, t12784: f64, t12855: f64, t17437: f64, t21121: f64, t21126: f64, t21129: f64, t21134: f64, t21137: f64, t21140: f64, t5304: f64, t5309: f64, t5313: f64, t5373: f64, t5391: f64, t6640: f64) -> f64 {
    let t21143 = t6601 * t1260;
    let t21146 = -0.2540682555144873302e-2_f64 * t5391 * t5304 - 0.28582678745379824648e-3_f64 * t12784 * t6640 - 0.85748036236139473944e-3_f64 * t12855 * t21121 - t17437 - 2.0_f64 / 81.0_f64 * t5373 * t5313 + t1222 * t21126 / 216.0_f64 - 7.0_f64 / 648.0_f64 * t1222 * t21129 + t5373 * t5309 / 27.0_f64 - t1222 * t21134 / 144.0_f64 - t1222 * t21137 / 72.0_f64 - t1222 * t21140 / 48.0_f64 - 0.14291339372689912324e-3_f64 * t21143 * t1266;
    t21146
}
