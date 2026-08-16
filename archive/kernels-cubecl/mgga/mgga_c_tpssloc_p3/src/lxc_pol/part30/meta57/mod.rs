//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta57 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk392;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk393;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk394;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk395;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk396;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta57<F: Float>(t1088: F, t1090: F, t123: F, t1087: F, t423: F, t419: F, t409: F, t410: F, t1086: F, t407: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1091, t1092, t1094, t1096, t1097, t1098) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk392::<F>(t1088, t1090, t123, t1087, t423, t419);
        let t1099 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk393::<F>(t1098, t409);
        let t1100 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk394::<F>(t410);
        let t1102 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk395::<F>(t1086, t1092);
        let (t1103, t1105, t1107) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk396::<F>(t1100, t1102, t1086, t407);
    (t1091, t1092, t1094, t1096, t1097, t1098, t1099, t1100, t1102, t1103, t1105, t1107)
}
