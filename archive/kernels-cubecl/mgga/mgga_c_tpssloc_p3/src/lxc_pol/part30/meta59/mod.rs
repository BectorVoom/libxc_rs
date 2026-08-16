//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta59 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk401;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk402;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk403;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk404;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta59<F: Float>(t1117: F, t1118: F, t1099: F, t1086: F, t1092: F, t432: F, t427: F, t1111: F, t1103: F, t1108: F, t1115: F, t435: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1119, t1121, t1122, t1124, t1127, t1128) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk401::<F>(t1117, t1118, t1099, t1086, t1092, t432);
        let t1129 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk402::<F>(t1128, t427);
        let (t1131, t1134, t1136) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk403::<F>(t1086, t1111, t1092, t1103, t1108, t1115);
        let t1137 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk404::<F>(t435);
    (t1119, t1121, t1122, t1124, t1127, t1128, t1129, t1131, t1134, t1136, t1137)
}
