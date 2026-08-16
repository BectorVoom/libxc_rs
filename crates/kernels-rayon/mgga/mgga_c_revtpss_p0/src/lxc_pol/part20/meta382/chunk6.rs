//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1394/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1394(t136: f64, t2457: f64, t2710: f64, t2760: f64, t10073: f64, t10929: f64, t10069: f64, t10654: f64, t2790: f64, t9292: f64, t10932: f64, t2754: f64, t2811: f64, t40340: f64, t40927: f64, t40938: f64, t40942: f64, t40945: f64, t40948: f64, t4514: f64, t820: f64, t837: f64) -> f64 {
    let t40952 = t2710 * t2760 * t136 * t2457;
    let t40954 = t10073 * t10929;
    let t40956 = t10069 * t10654;
    let t40958 = t9292 * t2790;
    let t40960 = -0.26341796731742046395e1_f64 * t820 * t40927 * t837 - 0.39512695097613069592e1_f64 * t4514 * t10932 * t2754 + 0.39512695097613069591e1_f64 * t820 * t2811 * t40340 - 0.39029762157531132075e-2_f64 * t40938 + 0.39029762157531132076e-1_f64 * t40942 - 0.18505311230957427423e-1_f64 * t40945 - 0.78059524315062264152e-1_f64 * t40948 + 0.69394917116090352835e-2_f64 * t40952 + 0.7805952431506226415e-2_f64 * t40954 + 0.87805989105806821314e-1_f64 * t40956 - 0.68293547082294194357e-1_f64 * t40958;
    t40960
}
