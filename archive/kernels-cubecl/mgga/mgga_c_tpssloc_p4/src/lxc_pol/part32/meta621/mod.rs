//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta621 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2026;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta621<F: Float>(t27551: F, t7327: F, t135: F, t24847: F, t7284: F, t1090: F, t24821: F, t1089: F, t1235: F, t11708: F, t24728: F, t11713: F, t11715: F, t11717: F, sigma2: F, t24649: F, t24658: F, t2131: F, t82985: F, t24727: F, t24732: F, t7337: F, t11835: F, t7310: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t86077, t86094, t86102, t86116, t86140, t86146) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2026::<F>(t27551, t7327, t135, t24847, t7284, t1090, t24821, t1089, t1235, t11708, t24728, t11713, t11715, t11717, sigma2);
        let (t86149, t86154, t86164, t86167, t86171, t86184) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2027::<F>(t24649, t24658, t2131, t82985, t11713, t11717, t24727, t11708, t24732, t7337, t11835, t7310);
    (t86077, t86094, t86102, t86116, t86140, t86146, t86149, t86154, t86164, t86167, t86171, t86184)
}
