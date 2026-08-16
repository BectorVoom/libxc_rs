//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta650 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2262;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2263;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta650<F: Float>(t12461: F, t6995: F, t26161: F, t26163: F, t22581: F, t7685: F, t24987: F, t7000: F, t25985: F, t6876: F, t6514: F, t671: F, t1868: F, t2363: F, t5107: F, t652: F, t6534: F, t22574: F, t56198: F, t8643: F, t26162: F, t57802: F, t22597: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90034, t90036, t90038, t90040, t90041) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2262::<F>(t12461, t6995, t26161, t26163, t22581, t7685, t24987, t7000, t25985, t6876, t6514, t671);
        let (t90044, t90051, t90059, t90062, t90064) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2263::<F>(t1868, t2363, t5107, t652, t6534, t22574, t56198, t8643, t26162, t57802, t22597, t7685);
    (t90034, t90036, t90038, t90040, t90041, t90044, t90051, t90059, t90062, t90064)
}
