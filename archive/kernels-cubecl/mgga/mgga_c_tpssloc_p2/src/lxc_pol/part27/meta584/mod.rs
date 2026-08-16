//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2037;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2038;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta584<F: Float>(t22637: F, t81228: F, t81326: F, t22638: F, t81159: F, t22892: F, t6891: F, t80645: F, t6892: F, t81186: F, t22674: F, t22934: F, t6897: F, t22935: F, t6883: F, t22667: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F, t240: F, t656: F, t666: F, t2331: F, t625: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81328, t81350, t81365, t81375, t81379) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2037::<F>(t22637, t81228, t81326, t22638, t81159, t22892, t6891, t80645, t6892, t81186, t22674, t22934, t6897);
        let (t81393, t81395, t81399, t81438, t81439, t81440, t81442) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2038::<F>(t22935, t6883, t22667, t1987, t81144, t9537, t107, t835, t240, t656, t666, t2331, t625);
    (t81328, t81350, t81365, t81375, t81379, t81393, t81395, t81399, t81438, t81439, t81440, t81442)
}
