//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1178/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1178(t6071: f64, t72: f64, t686: f64, t2465: f64, t213: f64, t6041: f64, t6048: f64, t10995: f64, t10987: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11030: f64, t15018: f64, t15047: f64, t15050: f64, t887: f64) -> f64 {
    let t18796 = t6071 * t72;
    let t18797 = t18796 * t686;
    let t18798 = t2465 * t18797;
    let t18800 = t213 * t6041;
    let t18804 = t6048 * t72;
    let t18805 = t18804 * t686;
    let t18806 = t10995 * t18805;
    let t18810 = 0.23131639038696784278e-2_f64 * t15018 - t10987 - 0.73171657588172351096e-2_f64 * t11000 + 0.65049603595885220126e-3_f64 * t11004 - 0.9757440539382783019e-2_f64 * t18798 - 0.65854491829355115987e0_f64 * t18800 * t887 - 0.13009920719177044025e-1_f64 * t11013 + t11017 + 0.19514881078765566037e-1_f64 * t18806 + 0.11565819519348392139e-2_f64 * t11019 + t15047 + t15050 - 0.65049603595885220126e-3_f64 * t11030;
    t18810
}
