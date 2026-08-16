//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2162/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2162<F: Float>(t10035: F, t10090: F, t10102: F, t14120: F, t14149: F, t14161: F, t14166: F, t14171: F, t14203: F, t14221: F, t1437: F, t1883: F, t22316: F, t22321: F, t22858: F, t22863: F, t22912: F, t22954: F, t4114: F, t5767: F, t6844: F, t6862: F, t6874: F, t820: F) -> F {
    let t23019 = -F::cast_from(0.19756347548806534796e1_f64) * t820 * t5767 * t6844 + F::cast_from(0.19514881078765566038e-2_f64) * t14120 + t10035 - F::cast_from(0.21951497276451705329e-1_f64) * t14149 + F::cast_from(0.34697458558045176417e-2_f64) * t14161 + F::cast_from(0.21951497276451705329e-1_f64) * t14166 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t22954 - F::cast_from(0.39512695097613069591e1_f64) * t820 * t10090 * t22858 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t4114 * t22863 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t5767 * t6874 - F::cast_from(0.65854491829355115987e0_f64) * t820 * t1437 * t22912 + F::cast_from(0.58544643236296698113e-1_f64) * t22316 - F::cast_from(0.19514881078765566038e-2_f64) * t14203 - F::cast_from(0.19756347548806534796e1_f64) * t820 * t22321 * t1883 + F::cast_from(0.39512695097613069591e1_f64) * t820 * t14171 * t6862 - F::cast_from(0.34697458558045176417e-2_f64) * t14221 + t10102;
    t23019
}
