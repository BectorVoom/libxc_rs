//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta577 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1860;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1861;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta577<F: Float>(t1516: F, t81763: F, t23083: F, t25094: F, t1510: F, t2379: F, t25119: F, t815: F, t2631: F, t47285: F, t6605: F, t9972: F, t12971: F, t1894: F, t236: F, t6591: F, t23046: F, t4184: F, t812: F, t836: F, t13080: F, t23146: F, t242: F, t81816: F) -> (F, F, F, F, F, F, F, F) {
        let (t87345, t87347, t87351, t87355) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1860::<F>(t1516, t81763, t23083, t25094, t1510, t2379, t25119, t815, t2631, t47285, t6605, t9972);
        let (t87359, t87363, t87365, t87368) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1861::<F>(t12971, t1894, t236, t6591, t23046, t4184, t812, t836, t13080, t23146, t242, t81816);
    (t87345, t87347, t87351, t87355, t87359, t87363, t87365, t87368)
}
