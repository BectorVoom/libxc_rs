//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1394/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1394<F: Float>(t136: F, t2457: F, t2710: F, t2760: F, t10073: F, t10929: F, t10069: F, t10654: F, t2790: F, t9292: F, t10932: F, t2754: F, t2811: F, t40340: F, t40927: F, t40938: F, t40942: F, t40945: F, t40948: F, t4514: F, t820: F, t837: F) -> F {
    let t40952 = t2710 * t2760 * t136 * t2457;
    let t40954 = t10073 * t10929;
    let t40956 = t10069 * t10654;
    let t40958 = t9292 * t2790;
    let t40960 = -F::cast_from(0.26341796731742046395e1_f64) * t820 * t40927 * t837 - F::cast_from(0.39512695097613069592e1_f64) * t4514 * t10932 * t2754 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t2811 * t40340 - F::cast_from(0.39029762157531132075e-2_f64) * t40938 + F::cast_from(0.39029762157531132076e-1_f64) * t40942 - F::cast_from(0.18505311230957427423e-1_f64) * t40945 - F::cast_from(0.78059524315062264152e-1_f64) * t40948 + F::cast_from(0.69394917116090352835e-2_f64) * t40952 + F::cast_from(0.7805952431506226415e-2_f64) * t40954 + F::cast_from(0.87805989105806821314e-1_f64) * t40956 - F::cast_from(0.68293547082294194357e-1_f64) * t40958;
    t40960
}
