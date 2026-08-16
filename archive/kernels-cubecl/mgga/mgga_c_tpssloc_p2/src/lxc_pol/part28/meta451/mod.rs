//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta451 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1645;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1646;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta451<F: Float>(t22692: F, t3851: F, t7208: F, t22717: F, t22725: F, t1332: F, t1336: F, t2089: F, t22697: F, t22701: F, t22707: F, t22721: F, t22728: F, t22730: F, t3773: F, t3777: F, t7209: F, t7211: F, t1338: F, t7191: F, t1352: F, t24063: F, t553: F, t2085: F, t3787: F, t3793: F, t3856: F, t22735: F, t22743: F, t22745: F, t22749: F, t22752: F, t22884: F, t22888: F, t22895: F, t22900: F, t544: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t24099, t24103, t24108, t24110, t24115) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1645::<F>(t22692, t3851, t7208, t22717, t22725, t1332, t1336, t2089, t22697, t22701, t22707, t22721, t22728, t22730, t3773, t3777, t7209, t7211);
        let (t24116, t24117, t24121, t24128, t24131, t24137) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1646::<F>(t1338, t7191, t1352, t24063, t553, t2085, t3787, t3793, t3856, t7208, t1336, t22735, t22743, t22745, t22749, t22752, t22884, t22888, t22895, t22900, t544);
    (t24099, t24103, t24108, t24110, t24115, t24116, t24117, t24121, t24128, t24131, t24137)
}
