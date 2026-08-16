//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1941/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1941(t11877: f64, t11881: f64, t1244: f64, t1249: f64, t14986: f64, t14989: f64, t14992: f64, t14997: f64, t15001: f64, t15004: f64, t15009: f64, t15016: f64, t15019: f64, t15023: f64, t15027: f64, t1729: f64, t1756: f64, t3604: f64, t3610: f64, t3613: f64, t3617: f64, t3624: f64, t3628: f64, t4964: f64, t5064: f64, t5073: f64) -> f64 {
    let t15030 = t11877 * t1756 + 6.0_f64 * t11881 * t15001 + t1244 * t14986 + 2.0_f64 * t1244 * t14989 + t1244 * t15016 + 2.0_f64 * t1249 * t4964 - 2.0_f64 * t14992 * t3624 + 4.0_f64 * t14997 * t3610 + 4.0_f64 * t15004 * t3610 + 2.0_f64 * t15009 * t3610 - t15019 * t3624 - t15023 * t3624 + 2.0_f64 * t15027 * t3613 + t1729 * t3628 + 2.0_f64 * t3604 * t5073 + 2.0_f64 * t3617 * t5064;
    t15030
}
