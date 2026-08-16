//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1280/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1280(t1011: f64, t15988: f64, t11672: f64, t11675: f64, t11881: f64, t11886: f64, t12004: f64, t15952: f64, t15959: f64, t15965: f64, t15970: f64, t15975: f64, t15986: f64, t1675: f64, t3091: f64, t3127: f64, t4783: f64, t4892: f64, t4899: f64) -> f64 {
    let t15990 = t1011 * t15988 / 216.0_f64;
    let t15991 = -0.28582678745379824648e-3_f64 * t3127 * t15952 + 0.28582678745379824648e-3_f64 * t11675 * t4783 + 0.28582678745379824648e-3_f64 * t3091 * t15959 - 0.28582678745379824648e-3_f64 * t3091 * t15965 + 0.28582678745379824648e-3_f64 * t4892 * t15970 - 0.14291339372689912324e-3_f64 * t4899 * t15975 + 0.48272968547752592739e-2_f64 * t12004 * t1675 - t11881 / 648.0_f64 - t11886 / 162.0_f64 - 0.15244095330869239812e-2_f64 * t11672 * t4783 + t15986 - t15990;
    t15991
}
