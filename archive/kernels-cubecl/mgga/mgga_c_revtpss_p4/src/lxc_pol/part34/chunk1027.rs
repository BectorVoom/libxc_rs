//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1027/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1027<F: Float>(t24042: F, t380: F, t5004: F, t6258: F, t1024: F, t11940: F, t12122: F, t12127: F, t1647: F, t16502: F, t16544: F, t16584: F, t1689: F, t1692: F, t19566: F, t23959: F, t24132: F, t24135: F, t24138: F, t24141: F, t24144: F, t24147: F, t24152: F, t24157: F, t3204: F, t3287: F, t3317: F, t342: F, t381: F, t4857: F, t6235: F, t6365: F, t6368: F, t6386: F, t6389: F) -> F {
    let t24162 = t380 * t24042;
    let t24167 = t5004 * t6258;
    let t24176 = -F::cast_from(0.39512695097613069591e1_f64) * t16544 * t6365 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t24132 - F::cast_from(0.19756347548806534796e1_f64) * t3287 * t24135 - F::cast_from(0.39512695097613069591e1_f64) * t12122 * t24138 + F::cast_from(0.19756347548806534796e1_f64) * t12127 * t24141 + F::cast_from(0.39512695097613069591e1_f64) * t3204 * t24144 - F::cast_from(0.39512695097613069591e1_f64) * t11940 * t24147 + F::cast_from(0.19756347548806534796e1_f64) * t6235 * t1692 - F::cast_from(0.19756347548806534796e1_f64) * t3317 * t24152 + F::cast_from(0.65854491829355115987e0_f64) * t23959 * t381 - F::cast_from(0.65854491829355115987e0_f64) * t1024 * t24157 + F::cast_from(0.19756347548806534796e1_f64) * t1647 * t6389 + F::cast_from(0.65854491829355115987e0_f64) * t342 * t24162 - F::cast_from(0.39512695097613069591e1_f64) * t16502 * t6365 - F::cast_from(0.19756347548806534796e1_f64) * t1024 * t24167 - F::cast_from(0.39512695097613069591e1_f64) * t4857 * t6368 + F::cast_from(0.19756347548806534796e1_f64) * t19566 * t1689 - F::cast_from(0.19756347548806534796e1_f64) * t16584 * t6386;
    t24176
}
