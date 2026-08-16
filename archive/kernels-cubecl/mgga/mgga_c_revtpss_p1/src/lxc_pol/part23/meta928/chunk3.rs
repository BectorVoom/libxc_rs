//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3024/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3024<F: Float>(t1082: F, t11940: F, t12146: F, t12154: F, t15655: F, t16544: F, t1689: F, t19414: F, t19463: F, t19492: F, t19549: F, t20136: F, t24126: F, t24132: F, t24157: F, t3223: F, t43443: F, t4967: F, t4970: F, t55330: F, t55764: F, t6368: F, t65216: F, t65220: F, t67825: F, t78740: F) -> F {
    let t80519 = F::cast_from(0.39512695097613069591e1_f64) * t43443 * t24126 + F::cast_from(0.19756347548806534796e1_f64) * t67825 * t1689 - F::cast_from(0.39512695097613069591e1_f64) * t15655 * t6368 - F::cast_from(0.39512695097613069591e1_f64) * t16544 * t20136 + F::cast_from(0.11853808529283920877e2_f64) * t65216 * t19549 - F::cast_from(0.11853808529283920877e2_f64) * t65220 * t19492 - F::cast_from(0.19756347548806534796e1_f64) * t12146 * t24132 - F::cast_from(0.19756347548806534796e1_f64) * t12154 * t24132 - F::cast_from(0.65854491829355115987e0_f64) * t3223 * t24157 - F::cast_from(0.19756347548806534796e1_f64) * t19463 * t4967 - F::cast_from(0.19756347548806534796e1_f64) * t19463 * t4970 - F::cast_from(0.11853808529283920877e2_f64) * t11940 * t1082 * t78740 - F::cast_from(0.11853808529283920877e2_f64) * t55330 * t55764 * t19414;
    t80519
}
