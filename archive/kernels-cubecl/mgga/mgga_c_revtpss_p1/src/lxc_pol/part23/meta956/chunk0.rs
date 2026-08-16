//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3191/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3191<F: Float>(t12809: F, t12916: F, t24839: F, t12787: F, t12866: F, t17448: F, t20770: F, t20838: F, t20933: F, t20937: F, t21008: F, t21017: F, t21046: F, t21310: F, t24232: F, t3625: F, t44517: F, t44521: F, t5405: F, t5407: F, t70014: F, t70819: F, t70917: F, t71047: F, t71061: F) -> F {
    let t83812 = t12809 * t12916 * t24839;
    let t83836 = -F::cast_from(0.17149607247227894789e-2_f64) * t70014 * t21310 + F::cast_from(0.42874018118069736972e-3_f64) * t83812 + F::cast_from(0.68598428988911579154e-2_f64) * t21017 * t20838 - F::cast_from(0.34299214494455789577e-2_f64) * t70917 * t21046 - F::cast_from(0.28582678745379824648e-3_f64) * t71047 - F::cast_from(0.85748036236139473944e-3_f64) * t44517 * t71061 * t20770 + F::cast_from(0.17149607247227894789e-2_f64) * t12866 * t71061 * t20937 - F::cast_from(0.17149607247227894789e-2_f64) * t44521 * t71061 * t20933 - F::cast_from(0.42874018118069736972e-3_f64) * t70819 * t5407 + F::cast_from(0.7145669686344956162e-3_f64) * t17448 * t21008 + F::cast_from(0.14291339372689912324e-2_f64) * t3625 * t12787 * t24232 * t5405;
    t83836
}
