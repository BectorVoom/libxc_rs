//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3191/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3191(t12809: f64, t12916: f64, t24839: f64, t12787: f64, t12866: f64, t17448: f64, t20770: f64, t20838: f64, t20933: f64, t20937: f64, t21008: f64, t21017: f64, t21046: f64, t21310: f64, t24232: f64, t3625: f64, t44517: f64, t44521: f64, t5405: f64, t5407: f64, t70014: f64, t70819: f64, t70917: f64, t71047: f64, t71061: f64) -> f64 {
    let t83812 = t12809 * t12916 * t24839;
    let t83836 = -0.17149607247227894789e-2_f64 * t70014 * t21310 + 0.42874018118069736972e-3_f64 * t83812 + 0.68598428988911579154e-2_f64 * t21017 * t20838 - 0.34299214494455789577e-2_f64 * t70917 * t21046 - 0.28582678745379824648e-3_f64 * t71047 - 0.85748036236139473944e-3_f64 * t44517 * t71061 * t20770 + 0.17149607247227894789e-2_f64 * t12866 * t71061 * t20937 - 0.17149607247227894789e-2_f64 * t44521 * t71061 * t20933 - 0.42874018118069736972e-3_f64 * t70819 * t5407 + 0.7145669686344956162e-3_f64 * t17448 * t21008 + 0.14291339372689912324e-2_f64 * t3625 * t12787 * t24232 * t5405;
    t83836
}
