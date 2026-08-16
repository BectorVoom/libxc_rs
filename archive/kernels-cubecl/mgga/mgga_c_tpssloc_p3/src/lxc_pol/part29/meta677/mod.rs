//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta677 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2268;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2269;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta677<F: Float>(t22561: F, t7458: F, t3652: F, t652: F, t7467: F, t22579: F, t7685: F, t1874: F, t55934: F, t12725: F, t6525: F, t26168: F, t6876: F, t25989: F, t83886: F, t25994: F, t4034: F, t15857: F, t1873: F, t45632: F, t26135: F, t3941: F, t671: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91759, t91762, t91763, t91765, t91767, t91769) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2268::<F>(t22561, t7458, t3652, t652, t7467, t22579, t7685, t1874, t55934, t12725, t6525, t26168, t6876);
        let (t91771, t91777, t91780, t91782, t91799) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2269::<F>(t25989, t83886, t25994, t4034, t15857, t1873, t652, t1874, t45632, t26135, t3941, t671);
    (t91759, t91762, t91763, t91765, t91767, t91769, t91771, t91777, t91780, t91782, t91799)
}
