//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3024/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3024(t1082: f64, t11940: f64, t12146: f64, t12154: f64, t15655: f64, t16544: f64, t1689: f64, t19414: f64, t19463: f64, t19492: f64, t19549: f64, t20136: f64, t24126: f64, t24132: f64, t24157: f64, t3223: f64, t43443: f64, t4967: f64, t4970: f64, t55330: f64, t55764: f64, t6368: f64, t65216: f64, t65220: f64, t67825: f64, t78740: f64) -> f64 {
    let t80519 = 0.39512695097613069591e1_f64 * t43443 * t24126 + 0.19756347548806534796e1_f64 * t67825 * t1689 - 0.39512695097613069591e1_f64 * t15655 * t6368 - 0.39512695097613069591e1_f64 * t16544 * t20136 + 0.11853808529283920877e2_f64 * t65216 * t19549 - 0.11853808529283920877e2_f64 * t65220 * t19492 - 0.19756347548806534796e1_f64 * t12146 * t24132 - 0.19756347548806534796e1_f64 * t12154 * t24132 - 0.65854491829355115987e0_f64 * t3223 * t24157 - 0.19756347548806534796e1_f64 * t19463 * t4967 - 0.19756347548806534796e1_f64 * t19463 * t4970 - 0.11853808529283920877e2_f64 * t11940 * t1082 * t78740 - 0.11853808529283920877e2_f64 * t55330 * t55764 * t19414;
    t80519
}
