//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta489 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1873;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta489<F: Float>(t1976: F, t4072: F, t671: F, t7670: F, t191: F, t192: F, t5118: F, t2020: F, t6997: F, t7685: F, t1390: F, t5187: F, t6878: F, t1983: F, t531: F, t1982: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t24980, t24983, t24987, t24988, t24989, t24990) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1873::<F>(t1976, t4072, t671, t7670, t191, t192, t5118, t2020, t6997, t7685, t1390, t5187);
        let (t24991, t24993, t24994, t24995) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1874::<F>(t24990, t6878, t1983, t192, t531, t1982);
    (t24980, t24983, t24987, t24988, t24989, t24990, t24991, t24993, t24994, t24995)
}
