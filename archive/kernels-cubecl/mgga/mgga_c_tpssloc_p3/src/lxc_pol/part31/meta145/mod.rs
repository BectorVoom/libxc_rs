//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta145 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk738;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk739;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk740;
use chunk3::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk741;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta145<F: Float>(t33: F, t3997: F, t1409: F, t2291: F, t3966: F, t634: F, t2298: F, t638: F, t607: F, t72: F, t1411: F, t1427: F, t1434: F, t3962: F, t3968: F, t3971: F, t3976: F, t609: F, t629: F, t642: F, t66: F, t80: F, t5: F, t1437: F, t2235: F, t2240: F, t3951: F, t3953: F, t3958: F, t605: F, t645: F, t86: F, t112: F, t111: F, t1441: F, t671: F, t89: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t3998, t4007, t4012, t4017, t4018, t4021) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk738::<F>(t33, t3997, t1409, t2291, t3966, t634, t2298, t638, t607, t72, t1411, t1427, t1434, t3962, t3968, t3971, t3976, t609, t629, t642, t66, t80);
        let (t4025, t4026) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk739::<F>(t5, t1437, t2235, t2240, t3951, t3953, t3958, t4021, t605, t645, t86, t112);
        let t4028 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk740::<F>(t111, t1441);
        let t4034 = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk741::<F>(t671, t89);
    (t3998, t4007, t4012, t4017, t4018, t4021, t4025, t4026, t4028, t4034)
}
