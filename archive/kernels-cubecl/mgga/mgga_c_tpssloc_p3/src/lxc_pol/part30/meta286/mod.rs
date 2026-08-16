//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta286 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1286;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1287;
use chunk2::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1288;
use chunk3::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1289;
use chunk4::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1290;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta286<F: Float>(t655: F, t93: F, t94: F, t101: F, t102: F, t195: F, t40: F, t197: F, t52: F, t138: F, t2409: F, t125: F, t2412: F, t701: F, t2414: F, t2393: F, t763: F, t2374: F, t702: F, t2411: F, t681: F, t141: F, t2413: F, t2508: F, t738: F, t2369: F, t745: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t9365, t9384, t9398, t9427, t9438, t9453) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1286::<F>(t655, t93, t94, t101, t102, t195, t40, t197, t52, t138, t2409, t125);
        let (t9454, t9457) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1287::<F>(t2412, t701, t2414, t9453);
        let (t9467, t9469, t9476) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1288::<F>(t2393, t763, t2374, t702, t9454, t2411);
        let t9484 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1289::<F>(t2409, t681, t125, t141, t2413, t9454);
        let (t9489, t9490) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1290::<F>(t2508, t738, t2369, t745);
    (t9365, t9384, t9398, t9427, t9438, t9457, t9467, t9469, t9476, t9484, t9489, t9490)
}
