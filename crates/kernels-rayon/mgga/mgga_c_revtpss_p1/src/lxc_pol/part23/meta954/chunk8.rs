//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3181/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3181(t24648: f64, t3172: f64, t3711: f64, t1214: f64, t24633: f64, t1261: f64, t24228: f64, t247: f64, t44895: f64, t1042: f64, t17569: f64, t20864: f64, t21184: f64, t21267: f64, t24644: f64, t3647: f64, t3719: f64, t5279: f64, t5302: f64, t5381: f64, t5384: f64, t57229: f64, t6635: f64, t69968: f64, t71585: f64, t80045: f64, t80050: f64) -> (f64, f64) {
    let t83539 = t3711 * t3172 * t24648;
    let t83551 = t24633 * t1214;
    let t83558 = t1261 * t247 * t44895 * t24228;
    let t83562 = 0.42874018118069736972e-3_f64 * t17569 * t21184 + 0.34299214494455789577e-2_f64 * t57229 * t6635 - 0.45732285992607719436e-2_f64 * t69968 * t5279 + 0.14291339372689912324e-2_f64 * t5381 * t20864 + 0.28582678745379824648e-3_f64 * t83539 + 0.71456696863449561621e-3_f64 * t3647 * t24644 + 0.71456696863449561621e-3_f64 * t1261 * t1042 * t5302 * t80050 + 0.71456696863449561621e-3_f64 * t1261 * t1042 * t5302 * t80045 + 0.42874018118069736972e-3_f64 * t5384 * t247 * t3719 * t83551 - 0.42344709252414555035e-3_f64 * t83558 - 0.38586616306262763276e-2_f64 * t71585 * t21267;
    (t83551, t83562)
}
