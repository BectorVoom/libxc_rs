//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1356/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1356<F: Float>(t14045: F, t3938: F, t3992: F, t2661: F, t1399: F, t5608: F, t5651: F, t10004: F, t14038: F, t14040: F, t14042: F, t14043: F, t9963: F, t9971: F, t9973: F, t9977: F, t9982: F) -> (F, F, F, F) {
    let t14046 = t14045 * t3938;
    let t14047 = t3992 * t14046;
    let t14049 = F::cast_from(0.57165357490759649296e-4_f64) * t2661 * t14047;
    let t14050 = t5608 * t1399;
    let t14051 = t3992 * t14050;
    let t14053 = F::cast_from(0.14291339372689912324e-4_f64) * t2661 * t14051;
    let t14054 = t5651 * t1399;
    let t14055 = t3992 * t14054;
    let t14057 = F::cast_from(0.57165357490759649296e-4_f64) * t2661 * t14055;
    let t14063 = -F::cast_from(0.80031500487063509016e-2_f64) * t9963 - t14038 - t14040 + t14042 + F::cast_from(0.13552000749142754193e-3_f64) * t14043 - t14049 + t14053 - t14057 - F::cast_from(0.12705000702321332056e-4_f64) * t9971 + F::cast_from(0.10003937560882938627e-2_f64) * t9973 + F::cast_from(0.27104001498285508387e-3_f64) * t9977 - F::cast_from(0.57165357490759649296e-4_f64) * t9982 + F::cast_from(0.25410001404642664112e-4_f64) * t10004;
    (t14046, t14050, t14054, t14063)
}
