//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta152 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk958;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk959;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta152<F: Float>(t3961: F, t4583: F, t4582: F, t2770: F, t3061: F, t1615: F, t376: F, t1022: F, t3131: F) -> (F, F, F, F, F, F, F) {
        let (t4584, t4585, t4588) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk958::<F>(t3961, t4583, t4582, t2770, t3061);
        let (t4589, t4590, t4593, t4594) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk959::<F>(t3961, t4588, t4582, t1615, t376, t1022, t3131);
    (t4584, t4585, t4588, t4589, t4590, t4593, t4594)
}
