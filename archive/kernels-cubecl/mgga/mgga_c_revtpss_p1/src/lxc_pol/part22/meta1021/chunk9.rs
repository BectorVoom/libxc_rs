//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3558/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3558<F: Float>(t3151: F, t6343: F, t1043: F, t1087: F, t1089: F, t12160: F, t15604: F, t15609: F, t16183: F, t16237: F, t16427: F, t16433: F, t16509: F, t16552: F, t16554: F, t16559: F, t16561: F, t1668: F, t1678: F, t19534: F, t19566: F, t20112: F, t20119: F, t20123: F, t3278: F, t3299: F, t3304: F, t3313: F, t42359: F, t43341: F, t43524: F, t55499: F, t55988: F, t6362: F, t65144: F, t66945: F, t67678: F) -> (F, F) {
    let t67869 = t6343 * t3151;
    let t67905 = -F::cast_from(0.13170898365871023197e1_f64) * t43341 * t67678 * t15604 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t20112 * t1043 * t1089 + F::cast_from(0.13170898365871023197e1_f64) * t3278 * t20119 + F::cast_from(0.13170898365871023197e1_f64) * t3299 * t67869 * t3304 - F::cast_from(0.52683593463484092788e1_f64) * t55988 * t16433 + F::cast_from(0.13170898365871023197e1_f64) * t42359 * t6362 + F::cast_from(0.26341796731742046394e1_f64) * t16509 * t16427 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t16237 * t1668 * t1089 + F::cast_from(0.79025390195226139182e1_f64) * t43524 * t67678 * t15609 - F::cast_from(0.15805078039045227836e2_f64) * t16559 * t55499 * t66945 + F::cast_from(0.39512695097613069591e1_f64) * t16552 * t65144 * t16554 - F::cast_from(0.39512695097613069591e1_f64) * t16559 * t65144 * t16561 + F::cast_from(0.65854491829355115987e0_f64) * t19566 * t3313 - F::cast_from(0.13170898365871023197e1_f64) * t12160 * t19534 + F::cast_from(0.26341796731742046394e1_f64) * t3278 * t20123 + F::cast_from(0.13170898365871023197e1_f64) * t1087 * t1678 * t16183 * t1089;
    (t67869, t67905)
}
