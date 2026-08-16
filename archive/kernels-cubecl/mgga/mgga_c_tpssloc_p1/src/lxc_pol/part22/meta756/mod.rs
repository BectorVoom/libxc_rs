//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta756 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2539;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2540;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta756<F: Float>(t136: F, t3297: F, t71193: F, t71197: F, t1113: F, t71168: F, t71172: F, t63911: F, t71144: F, t71400: F, t71403: F, t71406: F, t71408: F, t71411: F, t71414: F, t50846: F, t50854: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F, t71166: F, t71170: F, t71174: F, t71179: F, t21780: F, t3287: F, t1102: F, t3270: F, t21785: F, t43880: F, t18754: F, t4756: F, t14808: F, t5999: F, t18730: F, t4748: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t71417, t71420, t71423, t71426, t71428) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2539::<F>(t136, t3297, t71193, t71197, t1113, t71168, t71172, t63911, t71144, t71400, t71403, t71406, t71408, t71411, t71414);
        let t71440 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2540::<F>(t50846, t50854, t71146, t71150, t71152, t71154, t71156, t71160, t71166, t71170, t71174, t71179);
        let (t71446, t71449, t71452, t71454, t71456, t71458) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2541::<F>(t21780, t3287, t1102, t3270, t21785, t43880, t18754, t4756, t14808, t5999, t18730, t4748);
    (t71417, t71420, t71423, t71426, t71428, t71440, t71446, t71449, t71452, t71454, t71456, t71458)
}
