//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1089/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1089<F: Float>(t14050: F, t3992: F, t2661: F, t1399: F, t5651: F, t10004: F, t14038: F, t14040: F, t14042: F, t14043: F, t14049: F, t9963: F, t9971: F, t9973: F, t9977: F, t9982: F) -> (F,) {
    let t14051 = t3992 * t14050;
    let t14053 = 0.14291339372689912324e-4 * t2661 * t14051;
    let t14054 = t5651 * t1399;
    let t14055 = t3992 * t14054;
    let t14057 = 0.57165357490759649296e-4 * t2661 * t14055;
    let t14063 = -0.80031500487063509016e-2 * t9963 - t14038 - t14040 + t14042 + 0.13552000749142754193e-3 * t14043 - t14049 + t14053 - t14057 - 0.12705000702321332056e-4 * t9971 + 0.10003937560882938627e-2 * t9973 + 0.27104001498285508387e-3 * t9977 - 0.57165357490759649296e-4 * t9982 + 0.25410001404642664112e-4 * t10004;
    (t14063,)
}
