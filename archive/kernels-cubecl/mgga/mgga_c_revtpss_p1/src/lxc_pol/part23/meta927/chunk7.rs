//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3016/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3016<F: Float>(t1678: F, t19462: F, t1000: F, t16312: F, t16313: F, t16340: F, t16362: F, t16371: F, t16374: F, t16597: F, t16603: F, t19351: F, t19384: F, t19396: F, t19424: F, t20188: F, t20218: F, t23599: F, t23603: F, t24068: F, t24178: F, t3047: F, t3052: F, t3058: F, t3264: F, t42052: F, t4778: F, t4940: F, t5016: F, t53160: F, t6251: F, t6351: F, t6393: F, t64614: F, t68018: F, t79480: F, t996: F) -> F {
    let t80173 = t19462 * t1678;
    let t80211 = F::cast_from(0.11853808529283920877e2_f64) * t16603 * t64614 * t19424 - F::cast_from(0.39512695097613069591e1_f64) * t16312 * t16313 * t20218 - F::cast_from(0.19756347548806534796e1_f64) * t80173 * t1000 - F::cast_from(0.65854491829355115987e0_f64) * t3052 * t24178 - F::cast_from(0.39512695097613069591e1_f64) * t42052 * t24068 - F::cast_from(0.19756347548806534796e1_f64) * t16371 * t6393 + F::cast_from(0.13170898365871023197e1_f64) * t3058 * t996 * t79480 + F::cast_from(0.39512695097613069591e1_f64) * t16374 * t6251 + F::cast_from(0.39512695097613069591e1_f64) * t16340 * t6351 - F::cast_from(0.65854491829355115987e0_f64) * t3047 * t23599 + F::cast_from(0.39512695097613069591e1_f64) * t16597 * t6251 + F::cast_from(0.39512695097613069591e1_f64) * t4778 * t19396 + F::cast_from(0.39512695097613069591e1_f64) * t3264 * t23603 + F::cast_from(0.39512695097613069591e1_f64) * t16362 * t6351 - F::cast_from(0.39512695097613069592e1_f64) * t16312 * t68018 * t4940 - F::cast_from(0.39512695097613069592e1_f64) * t16312 * t16313 * t19384 - F::cast_from(0.19756347548806534796e1_f64) * t19351 * t5016 - F::cast_from(0.19756347548806534796e1_f64) * t16340 * t6393 - F::cast_from(0.11853808529283920877e2_f64) * t53160 * t20188;
    t80211
}
