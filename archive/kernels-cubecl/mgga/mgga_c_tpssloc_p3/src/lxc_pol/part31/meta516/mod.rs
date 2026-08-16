//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1713;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta516<F: Float>(t1530: F, t1649: F, t28: F, t5660: F, t191: F, t192: F, t6295: F, t1390: F, t6330: F, t1799: F, t1845: F, t6347: F, t5456: F, t576: F, t2031: F, t27956: F, t1860: F, t2032: F, t23963: F, t23995: F, t26016: F, t26911: F, t26920: F, t26936: F, t26948: F, t26954: F, t26960: F, t27937: F, t27961: F, t27966: F, t27972: F, t27976: F, t27979: F, t27982: F, t7026: F, t7428: F, t7432: F, t7435: F, t7782: F, t5: F, t109: F, t112: F, t23912: F, t26127: F, t28012: F, t28014: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28792, t28795, t28821, t28826, t28830, t28834) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1712::<F>(t1530, t1649, t28, t5660, t191, t192, t6295, t1390, t6330, t1799, t1845, t6347);
        let (t28893, t28935, t28941) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1713::<F>(t5456, t576, t2031, t27956, t1860, t2032, t23963, t23995, t26016, t26911, t26920, t26936, t26948, t26954, t26960, t27937, t27961, t27966, t27972, t27976, t27979, t27982, t7026, t7428, t7432, t7435, t7782);
        let (t28942, t28943, t28951) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1714::<F>(t5, t109, t28941, t112, t23912, t26127, t28012, t28014);
    (t28792, t28795, t28821, t28826, t28830, t28834, t28893, t28935, t28942, t28943, t28951)
}
