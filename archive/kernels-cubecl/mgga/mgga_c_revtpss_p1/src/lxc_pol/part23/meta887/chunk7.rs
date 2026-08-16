//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2808/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2808<F: Float>(t1580: F, t18316: F, t689: F, t14480: F, t252: F, t2782: F, t6071: F, t11008: F, t23384: F, t23404: F, t2765: F, t40988: F, t40998: F, t4533: F, t50236: F, t50245: F, t50248: F, t50253: F, t6048: F, t61411: F, t61419: F, t61422: F, t61430: F, t61437: F, t865: F) -> F {
    let t76020 = t689 * t18316 * t1580;
    let t76026 = t2782 * t252 * t14480 * t6071;
    let t76038 = -F::cast_from(0.34697458558045176418e-2_f64) * t61411 - F::cast_from(0.11853808529283920877e2_f64) * t865 * t11008 * t6048 * t4533 - F::cast_from(0.19514881078765566038e-2_f64) * t50236 + F::cast_from(0.16463622957338778997e-1_f64) * t76020 + F::cast_from(0.98781737744032673976e-1_f64) * t61419 - F::cast_from(0.65854491829355115984e-1_f64) * t61422 - F::cast_from(0.32927245914677557992e-1_f64) * t76026 - F::cast_from(0.17073386770573548589e-1_f64) * t40988 + F::cast_from(0.58544643236296698113e-1_f64) * t61430 + F::cast_from(0.19514881078765566038e-2_f64) * t50245 + F::cast_from(0.33133632253434461091e-3_f64) * t50248 - t40998 + F::cast_from(0.58544643236296698114e-1_f64) * t61437 - F::cast_from(0.65854491829355115987e0_f64) * t2765 * t23384 + F::cast_from(0.39512695097613069591e1_f64) * t2765 * t23404 - F::cast_from(0.78059524315062264152e-1_f64) * t50253;
    t76038
}
