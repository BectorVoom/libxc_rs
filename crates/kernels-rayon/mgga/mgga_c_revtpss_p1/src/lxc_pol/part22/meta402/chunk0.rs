//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1994/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1994(t14045: f64, t3938: f64, t3992: f64, t2661: f64, t1399: f64, t5608: f64, t5651: f64, t10004: f64, t14038: f64, t14040: f64, t14042: f64, t14043: f64, t9963: f64, t9971: f64, t9973: f64, t9977: f64, t9982: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14046 = t14045 * t3938;
    let t14047 = t3992 * t14046;
    let t14049 = 0.57165357490759649296e-4_f64 * t2661 * t14047;
    let t14050 = t5608 * t1399;
    let t14051 = t3992 * t14050;
    let t14053 = 0.14291339372689912324e-4_f64 * t2661 * t14051;
    let t14054 = t5651 * t1399;
    let t14055 = t3992 * t14054;
    let t14057 = 0.57165357490759649296e-4_f64 * t2661 * t14055;
    let t14063 = -0.80031500487063509016e-2_f64 * t9963 - t14038 - t14040 + t14042 + 0.13552000749142754193e-3_f64 * t14043 - t14049 + t14053 - t14057 - 0.12705000702321332056e-4_f64 * t9971 + 0.10003937560882938627e-2_f64 * t9973 + 0.27104001498285508387e-3_f64 * t9977 - 0.57165357490759649296e-4_f64 * t9982 + 0.25410001404642664112e-4_f64 * t10004;
    (t14047, t14049, t14051, t14053, t14055, t14057, t14063)
}
