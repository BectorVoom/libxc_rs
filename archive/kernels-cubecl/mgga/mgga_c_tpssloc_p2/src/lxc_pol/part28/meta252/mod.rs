//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta252 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1092;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1093;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1094;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1095;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta252<F: Float>(t1351: F, t562: F, t550: F, t6976: F, t1992: F, t1372: F, t1998: F, t214: F, t1985: F, t1388: F, t3701: F, t33: F, t63: F, t2240: F, t625: F, t67: F, t1864: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t6978, t6979, t6980, t6982, t6983, t6984, t6999, t7025) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1092::<F>(t1351, t562, t550, t6976, t1992, t1372, t1998, t214, t1985, t1388, t3701, t33, t63);
        let t7026 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1093::<F>(t2240, t7025);
        let t7031 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1094::<F>(t625, t67);
        let t7032 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1095::<F>(t1864, t7031);
    (t6978, t6979, t6980, t6982, t6983, t6984, t6999, t7025, t7026, t7031, t7032)
}
