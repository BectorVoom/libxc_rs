//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta646 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2373;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2374;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta646<F: Float>(t291: F, t48702: F, t48722: F, t10709: F, t4483: F, t10661: F, t10662: F, t1557: F, t10817: F, t14382: F, t14385: F, t42143: F, t10655: F, t14392: F, t14396: F, t42023: F, t2792: F, t2836: F, t4396: F, t14388: F, t2793: F, t10696: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t48725, t48727, t48730, t48732, t48734) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2373::<F>(t291, t48702, t48722, t10709, t4483, t10661, t10662, t1557, t10817, t14382, t14385, t42143);
        let (t48736, t48738, t48741, t48744, t48747) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2374::<F>(t10655, t14392, t14396, t42023, t2792, t2836, t4396, t10661, t14388, t2793, t10696, t1557);
    (t48725, t48727, t48730, t48732, t48734, t48736, t48738, t48741, t48744, t48747)
}
