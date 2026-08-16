//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta172 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk838;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk839;
use chunk2::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk840;
use chunk3::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta172<F: Float>(t2600: F, t541: F, t1329: F, t3726: F, t119: F, t3734: F, t210: F, t3719: F, t225: F, t3752: F, t554: F, t1332: F, t68: F, t1340: F, t1333: F, t1358: F, t1362: F, t1337: F, t551: F, t236: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3762, t3763, t3766, t3770, t3773) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk838::<F>(t2600, t541, t1329, t3726, t119, t3734, t210, t3719, t225, t3752);
        let (t3774, t3777) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk839::<F>(t3773, t554, t1332, t68);
        let (t3778, t3781, t3783, t3787) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk840::<F>(t1340, t3777, t1333, t1358, t1362, t1337, t551);
        let t3788 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk841::<F>(t236, t3787);
    (t3762, t3763, t3766, t3770, t3773, t3774, t3777, t3778, t3781, t3783, t3787, t3788)
}
