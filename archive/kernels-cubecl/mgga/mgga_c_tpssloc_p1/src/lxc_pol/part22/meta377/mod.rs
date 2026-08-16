//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta377 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1634;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta377<F: Float>(t2987: F, t5836: F, t2990: F, t5842: F, t13847: F, t4514: F, t2986: F, t17167: F, t4518: F, t17171: F, t10254: F, t5392: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t17800, t17801, t17804, t17805, t17808, t17809, t17811, t17814, t17817) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1634::<F>(t2987, t5836, t2990, t5842, t13847, t4514, t2986, t17167, t4518, t17171, t10254, t5392);
    (t17800, t17801, t17804, t17805, t17808, t17809, t17811, t17814, t17817)
}
