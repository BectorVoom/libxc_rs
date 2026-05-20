//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3181/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3181<F: Float>(t24648: F, t3172: F, t3711: F, t1214: F, t24633: F, t1261: F, t24228: F, t247: F, t44895: F, t1042: F, t17569: F, t20864: F, t21184: F, t21267: F, t24644: F, t3647: F, t3719: F, t5279: F, t5302: F, t5381: F, t5384: F, t57229: F, t6635: F, t69968: F, t71585: F, t80045: F, t80050: F) -> (F, F) {
    let t83539 = t3711 * t3172 * t24648;
    let t83551 = t24633 * t1214;
    let t83558 = t1261 * t247 * t44895 * t24228;
    let t83562 = F::cast_from(0.42874018118069736972e-3_f64) * t17569 * t21184 + F::cast_from(0.34299214494455789577e-2_f64) * t57229 * t6635 - F::cast_from(0.45732285992607719436e-2_f64) * t69968 * t5279 + F::cast_from(0.14291339372689912324e-2_f64) * t5381 * t20864 + F::cast_from(0.28582678745379824648e-3_f64) * t83539 + F::cast_from(0.71456696863449561621e-3_f64) * t3647 * t24644 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t1042 * t5302 * t80050 + F::cast_from(0.71456696863449561621e-3_f64) * t1261 * t1042 * t5302 * t80045 + F::cast_from(0.42874018118069736972e-3_f64) * t5384 * t247 * t3719 * t83551 - F::cast_from(0.42344709252414555035e-3_f64) * t83558 - F::cast_from(0.38586616306262763276e-2_f64) * t71585 * t21267;
    (t83551, t83562)
}
