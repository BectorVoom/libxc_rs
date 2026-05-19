//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 914/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk914<F: Float>(t11043: F, t786: F, t2467: F, t2828: F, t676: F, t123: F, t2465: F, t11004: F, t11010: F, t11013: F, t11017: F, t11019: F, t11022: F, t11026: F, t11030: F, t11032: F, t11037: F, t11040: F, t213: F, t257: F, t2765: F, t2772: F, t2829: F, t865: F) -> (F, F) {
    let t11044 = t786 * t11043;
    let t11045 = t11044 * t2467;
    let t11049 = t676 * t2828;
    let t11050 = t123 * t11049;
    let t11051 = t2465 * t11050;
    let t11053 = F::cast_from(0.19514881078765566038e-2_f64) * t11004 - F::cast_from(0.39512695097613069591e1_f64) * t865 * t11010 - F::cast_from(0.39029762157531132076e-1_f64) * t11013 + t11017 + F::cast_from(0.34697458558045176417e-2_f64) * t11019 + F::cast_from(0.29272321618148349057e-1_f64) * t11022 - F::cast_from(0.16463622957338778996e-1_f64) * t11026 - F::cast_from(0.19514881078765566038e-2_f64) * t11030 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t11032 * t257 - F::cast_from(0.32927245914677557992e-1_f64) * t11037 - t11040 + F::cast_from(0.39512695097613069591e1_f64) * t2765 * t2772 - F::cast_from(0.58544643236296698113e-1_f64) * t11045 - F::cast_from(0.19756347548806534796e1_f64) * t2765 * t2829 - F::cast_from(0.29272321618148349057e-1_f64) * t11051;
    (t11050, t11053)
}
