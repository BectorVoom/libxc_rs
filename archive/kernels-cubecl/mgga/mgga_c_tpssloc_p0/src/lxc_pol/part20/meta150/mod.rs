//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta150 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk956;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk957;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk958;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta150<F: Float>(t3551: F, t974: F, t1176: F, t3247: F, t2244: F, t3242: F, t3439: F, t225: F, t3481: F, t68: F, t484: F, t121: F, t486: F, t1216: F, t248: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3552, t3556, t3557, t3561, t3562, t3565) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk956::<F>(t3551, t974, t1176, t3247, t2244, t3242, t3439, t225, t3481);
        let (t3566, t3567, t3570) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk957::<F>(t3565, t68, t484, t121, t486);
        let t3572 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk958::<F>(t1216, t248, t3570);
    (t3552, t3556, t3557, t3561, t3562, t3565, t3566, t3567, t3570, t3572)
}
