//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta479 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1693;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta479<F: Float>(t4021: F, t79: F, t72: F, t1410: F, t2235: F, t3961: F, t605: F, t3967: F, t1433: F, t645: F, t1458: F, t649: F) -> (F, F, F, F, F, F, F) {
        let (t26066, t26067, t26070, t26073, t26076, t26090, t26114) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1693::<F>(t4021, t79, t72, t1410, t2235, t3961, t605, t3967, t1433, t645, t1458, t649);
    (t26066, t26067, t26070, t26073, t26076, t26090, t26114)
}
