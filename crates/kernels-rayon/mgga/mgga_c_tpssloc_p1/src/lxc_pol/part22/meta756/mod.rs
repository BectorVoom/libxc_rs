//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2539;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2540;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta756(t136: f64, t3297: f64, t71193: f64, t71197: f64, t1113: f64, t71168: f64, t71172: f64, t63911: f64, t71144: f64, t71400: f64, t71403: f64, t71406: f64, t71408: f64, t71411: f64, t71414: f64, t50846: f64, t50854: f64, t71146: f64, t71150: f64, t71152: f64, t71154: f64, t71156: f64, t71160: f64, t71166: f64, t71170: f64, t71174: f64, t71179: f64, t21780: f64, t3287: f64, t1102: f64, t3270: f64, t21785: f64, t43880: f64, t18754: f64, t4756: f64, t14808: f64, t5999: f64, t18730: f64, t4748: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t71417, t71420, t71423, t71426, t71428) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2539(t136, t3297, t71193, t71197, t1113, t71168, t71172, t63911, t71144, t71400, t71403, t71406, t71408, t71411, t71414);
        let t71440 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2540(t50846, t50854, t71146, t71150, t71152, t71154, t71156, t71160, t71166, t71170, t71174, t71179);
        let (t71446, t71449, t71452, t71454, t71456, t71458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2541(t21780, t3287, t1102, t3270, t21785, t43880, t18754, t4756, t14808, t5999, t18730, t4748);
    (t71417, t71420, t71423, t71426, t71428, t71440, t71446, t71449, t71452, t71454, t71456, t71458)
}
