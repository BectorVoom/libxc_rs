//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1764;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1765;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1766;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta472<F: Float>(t381: F, t883: F, t6743: F, t23384: F, t6790: F, t6733: F, t6796: F, t995: F, t6802: F, t614: F, t6794: F, t131: F, t350: F, t23602: F, t3127: F, t1011: F, t3131: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23634, t23635) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1764::<F>(t381, t883, t6743);
        let (t23642, t23657, t23665) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1765::<F>(t23384, t6790, t6733, t6743, t6796, t995);
        let (t23666, t23668, t23669, t23670, t23677, t23678) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1766::<F>(t23665, t6802, t614, t6794, t131, t350, t23602, t3127, t1011, t3131);
    (t23634, t23635, t23642, t23657, t23665, t23666, t23668, t23669, t23670, t23677, t23678)
}
