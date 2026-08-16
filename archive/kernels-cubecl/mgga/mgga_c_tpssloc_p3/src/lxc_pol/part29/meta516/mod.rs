//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1890;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1891;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta516<F: Float>(t3: F, t3966: F, t1484: F, t1530: F, t16596: F, t1877: F, t1915: F, t193: F, t202: F, t23290: F, t23295: F, t2522: F, t25353: F, t25358: F, t25365: F, t25374: F, t4119: F, t4255: F, t4303: F, t4314: F, t6666: F, t6670: F, t7541: F, t776: F, t868: F, t870: F, t28: F, t23788: F, t1081: F, t10143: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t25588, t25882) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1890::<F>(t3, t3966, t1484, t1530, t16596, t1877, t1915, t193, t202, t23290, t23295, t2522, t25353, t25358, t25365, t25374, t4119, t4255, t4303, t4314, t6666, t6670, t7541, t776, t868, t870);
        let (t25891, t25892, t25898, t25901, t25905, t25921, t25927) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1891::<F>(t28, t870, t4255, t16596, t23788, t1081, t1484, t4119, t25365, t10143);
    (t25588, t25882, t25891, t25892, t25898, t25901, t25905, t25921, t25927)
}
