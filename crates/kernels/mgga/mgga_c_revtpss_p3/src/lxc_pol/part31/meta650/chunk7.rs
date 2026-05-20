//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2151/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2151<F: Float>(t19826: F, t25509: F, t20029: F, t25505: F, t100074: F, t100255: F, t1671: F, t19651: F, t19663: F, t19668: F, t19672: F, t19930: F, t19934: F, t27536: F, t4875: F, t6312: F, t7132: F, t93655: F) -> F {
    let t107015 = t25509 * t19826;
    let t107027 = t25505 * t20029;
    let t107035 = F::cast_from(0.22866142996303859718e-2_f64) * t93655 * t6312 - F::cast_from(0.28582678745379824648e-3_f64) * t107015 - F::cast_from(0.28582678745379824648e-2_f64) * t7132 * t19663 + F::cast_from(0.95275595817932748826e-3_f64) * t7132 * t19668 + F::cast_from(0.1270341277572436651e-2_f64) * t7132 * t19672 + F::cast_from(0.57165357490759649296e-3_f64) * t27536 * t19651 - F::cast_from(0.57165357490759649296e-3_f64) * t100255 * t4875 + F::cast_from(0.57165357490759649296e-3_f64) * t107027 + F::cast_from(0.17149607247227894789e-2_f64) * t7132 * t19930 - F::cast_from(0.11433071498151929859e-2_f64) * t7132 * t19934 - F::cast_from(0.45732285992607719437e-2_f64) * t100074 * t1671;
    t107035
}
