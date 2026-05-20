//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3020/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3020<F: Float>(t12050: F, t357: F, t11631: F, t6299: F, t1043: F, t11940: F, t12047: F, t12052: F, t16502: F, t16552: F, t16559: F, t16560: F, t16566: F, t19450: F, t19456: F, t19502: F, t20123: F, t20139: F, t20146: F, t43341: F, t43438: F, t4866: F, t4954: F, t5004: F, t5012: F, t55499: F, t55646: F, t55887: F, t6235: F, t6365: F, t78496: F, t80277: F, t80312: F, t80341: F, t999: F) -> F {
    let t80350 = t12050 * t357;
    let t80358 = t11631 * t6299;
    let t80391 = F::cast_from(0.19756347548806534796e1_f64) * t16566 * t19450 * t80350 * t4866 + F::cast_from(0.79025390195226139182e1_f64) * t43438 * t80312 * t19502 + F::cast_from(0.11853808529283920877e2_f64) * t16552 * t55499 * t80358 * t1043 - F::cast_from(0.11853808529283920877e2_f64) * t16559 * t55499 * t80277 * t1043 - F::cast_from(0.11853808529283920877e2_f64) * t11940 * t5004 * t19456 + F::cast_from(0.39512695097613069591e1_f64) * t4954 * t20123 - F::cast_from(0.39512695097613069591e1_f64) * t55646 * t6365 - F::cast_from(0.39512695097613069591e1_f64) * t16502 * t20146 - F::cast_from(0.11853808529283920877e2_f64) * t16559 * t19450 * t16560 * t4866 + F::cast_from(0.39512695097613069591e1_f64) * t55887 * t20139 - F::cast_from(0.65854491829355115987e0_f64) * t43341 * t78496 * t80350 * t999 + F::cast_from(0.65854491829355115987e0_f64) * t12047 * t80341 * t12052 + F::cast_from(0.19756347548806534796e1_f64) * t6235 * t5012;
    t80391
}
