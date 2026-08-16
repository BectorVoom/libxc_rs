//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk861;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk862;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta169<F: Float>(t381: F, t4552: F, t1049: F, t1603: F, t1604: F, t225: F, t1625: F, t990: F, t4343: F, t977: F, t2979: F, t4338: F, t1539: F, t248: F, t3051: F) -> (F, F, F, F, F, F, F) {
        let (t4553, t4555, t4557) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk861::<F>(t381, t4552, t1049, t1603, t1604, t225);
        let (t4559, t4562, t4565, t4571) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk862::<F>(t1625, t990, t4343, t977, t2979, t4338, t1539, t248, t3051);
    (t4553, t4555, t4557, t4559, t4562, t4565, t4571)
}
