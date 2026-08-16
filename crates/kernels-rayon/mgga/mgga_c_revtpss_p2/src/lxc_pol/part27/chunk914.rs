//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 914/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk914(t11043: f64, t786: f64, t2467: f64, t2828: f64, t676: f64, t123: f64, t2465: f64, t11004: f64, t11010: f64, t11013: f64, t11017: f64, t11019: f64, t11022: f64, t11026: f64, t11030: f64, t11032: f64, t11037: f64, t11040: f64, t213: f64, t257: f64, t2765: f64, t2772: f64, t2829: f64, t865: f64) -> (f64, f64) {
    let t11044 = t786 * t11043;
    let t11045 = t11044 * t2467;
    let t11049 = t676 * t2828;
    let t11050 = t123 * t11049;
    let t11051 = t2465 * t11050;
    let t11053 = 0.19514881078765566038e-2_f64 * t11004 - 0.39512695097613069591e1_f64 * t865 * t11010 - 0.39029762157531132076e-1_f64 * t11013 + t11017 + 0.34697458558045176417e-2_f64 * t11019 + 0.29272321618148349057e-1_f64 * t11022 - 0.16463622957338778996e-1_f64 * t11026 - 0.19514881078765566038e-2_f64 * t11030 + 0.65854491829355115987e0_f64 * t213 * t11032 * t257 - 0.32927245914677557992e-1_f64 * t11037 - t11040 + 0.39512695097613069591e1_f64 * t2765 * t2772 - 0.58544643236296698113e-1_f64 * t11045 - 0.19756347548806534796e1_f64 * t2765 * t2829 - 0.29272321618148349057e-1_f64 * t11051;
    (t11050, t11053)
}
