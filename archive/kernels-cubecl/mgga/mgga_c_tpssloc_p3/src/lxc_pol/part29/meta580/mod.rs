//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1998;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1999;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta580<F: Float>(t22705: F, t22852: F, t550: F, t80786: F, t22823: F, t281: F, t22855: F, t3862: F, t6940: F, t1358: F, t22836: F, t22690: F, t3787: F, t3792: F, t236: F, t3850: F, t1361: F, t22792: F, t3719: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t80789, t80791, t80792, t80794, t80796, t80798) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1998::<F>(t22705, t22852, t550, t80786, t22823, t281, t22855, t3862, t6940, t1358, t22836, t22690, t3787);
        let (t80801, t80807, t80814) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1999::<F>(t22852, t3792, t80786, t80798, t22705, t236, t3850, t550, t1361, t22690, t22792, t3719);
    (t80789, t80791, t80792, t80794, t80796, t80798, t80801, t80807, t80814)
}
