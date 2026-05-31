//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3117/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3117<F: Float>(t1168: F, t12423: F, t12429: F, t12470: F, t12511: F, t17032: F, t1744: F, t1745: F, t20520: F, t20542: F, t20612: F, t20618: F, t20622: F, t20626: F, t24331: F, t24366: F, t24417: F, t24420: F, t3452: F, t3477: F, t45085: F, t5142: F, t5143: F, t58005: F, t58304: F, t6487: F, t6502: F, t6506: F, t69411: F, t69565: F) -> F {
    let t82045 = -F::cast_from(0.57895126195293126241e3_f64) * t58304 * t20612 + F::cast_from(0.1929837539843104208e3_f64) * t17032 * t20622 + F::cast_from(3.0_f64) * t69565 * t1745 + F::cast_from(3.0_f64) * t20542 * t5143 + F::cast_from(18.0_f64) * t3477 * t6487 * t5142 - F::cast_from(6.0_f64) * t12511 * t24417 - F::cast_from(6.0_f64) * t3452 * t5143 * t6502 - F::cast_from(6.0_f64) * t3452 * t1745 * t20520 + F::cast_from(0.96491876992155210402e2_f64) * t12423 * t24420 + F::cast_from(0.96491876992155210402e2_f64) * t3477 * t69411 * t1744 + F::cast_from(0.96491876992155210402e2_f64) * t3477 * t20618 * t5142 + F::cast_from(0.62071215503128080361e4_f64) * t58005 * t20626 + F::cast_from(0.11579025239058625248e4_f64) * t12470 * t24331 * t1168 - F::cast_from(0.57895126195293126243e3_f64) * t12429 * t6506 * t5142 - F::cast_from(0.24828486201251232145e5_f64) * t45085 * t24366 * t1168;
    t82045
}
