//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta138 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk716;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk717;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk718;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk719;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta138<F: Float>(t2600: F, t541: F, t1329: F, t3726: F, t1332: F, t68: F, t1340: F, t1333: F, t1358: F, t1362: F, t1337: F, t551: F, t236: F, t240: F, t1336: F, t550: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t3762, t3763, t3777) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk716::<F>(t2600, t541, t1329, t3726, t1332, t68);
        let (t3778, t3781, t3783, t3787) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk717::<F>(t1340, t3777, t1333, t1358, t1362, t1337, t551);
        let t3788 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk718::<F>(t236, t3787);
        let (t3789, t3790, t3792) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk719::<F>(t240, t3788, t1336, t550);
    (t3762, t3763, t3777, t3778, t3781, t3783, t3787, t3788, t3789, t3790, t3792)
}
