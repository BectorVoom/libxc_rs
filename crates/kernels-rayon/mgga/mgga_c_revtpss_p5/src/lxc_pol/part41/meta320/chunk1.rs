//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1099/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1099(t1412: f64, t1882: f64, t3938: f64, t3992: f64, t2661: f64, t1399: f64, t5608: f64, t5651: f64, t5774: f64, t72: f64, t686: f64, t3915: f64) -> (f64, f64, f64, f64, f64) {
    let t14045 = t1412 * t1882;
    let t14046 = t14045 * t3938;
    let t14047 = t3992 * t14046;
    let t14049 = 0.57165357490759649296e-4_f64 * t2661 * t14047;
    let t14050 = t5608 * t1399;
    let t14051 = t3992 * t14050;
    let t14053 = 0.14291339372689912324e-4_f64 * t2661 * t14051;
    let t14054 = t5651 * t1399;
    let t14055 = t3992 * t14054;
    let t14057 = 0.57165357490759649296e-4_f64 * t2661 * t14055;
    let t14078 = t5774 * t72;
    let t14079 = t14078 * t686;
    let t14081 = 0.19514881078765566038e-1_f64 * t3915 * t14079;
    (t14045, t14049, t14053, t14057, t14081)
}
