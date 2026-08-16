//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta205 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk952;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta205<F: Float>(t1043: F, t2775: F, t3961: F, t4582: F, t2770: F, t3061: F, t1615: F, t376: F) -> (F, F, F, F, F, F, F) {
        let (t4583, t4584, t4585, t4588, t4589, t4590, t4593) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk952::<F>(t1043, t2775, t3961, t4582, t2770, t3061, t1615, t376);
    (t4583, t4584, t4585, t4588, t4589, t4590, t4593)
}
