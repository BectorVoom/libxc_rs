//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta498 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta498<F: Float>(t25470: F, t6786: F, t1539: F, t23685: F, t6784: F, t23657: F, t7610: F, t23327: F, t23346: F, t23619: F, t23626: F, t23629: F, t25456: F, t25459: F, t25465: F, t25467: F, t6687: F, t6797: F, t7607: F) -> (F, F, F, F, F) {
        let (t25471, t25475, t25476, t25479, t25482) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1814::<F>(t25470, t6786, t1539, t23685, t6784, t23657, t7610, t23327, t23346, t23619, t23626, t23629, t25456, t25459, t25465, t25467, t6687, t6797, t7607);
    (t25471, t25475, t25476, t25479, t25482)
}
