//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1517;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta330<F: Float>(t16111: F, t3739: F, t12225: F, t16095: F, t2586: F, t1338: F, t5318: F, t3866: F, t5310: F, t3799: F, t5289: F, t2371: F, t5154: F) -> (F, F, F, F, F, F, F) {
        let (t16113, t16118, t16119, t16132, t16147, t16159, t16164) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1517::<F>(t16111, t3739, t12225, t16095, t2586, t1338, t5318, t3866, t5310, t3799, t5289, t2371, t5154);
    (t16113, t16118, t16119, t16132, t16147, t16159, t16164)
}
