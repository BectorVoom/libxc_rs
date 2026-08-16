//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta516 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1712;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1713;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1714;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta516(t1530: f64, t1649: f64, t28: f64, t5660: f64, t191: f64, t192: f64, t6295: f64, t1390: f64, t6330: f64, t1799: f64, t1845: f64, t6347: f64, t5456: f64, t576: f64, t2031: f64, t27956: f64, t1860: f64, t2032: f64, t23963: f64, t23995: f64, t26016: f64, t26911: f64, t26920: f64, t26936: f64, t26948: f64, t26954: f64, t26960: f64, t27937: f64, t27961: f64, t27966: f64, t27972: f64, t27976: f64, t27979: f64, t27982: f64, t7026: f64, t7428: f64, t7432: f64, t7435: f64, t7782: f64, t5: f64, t109: f64, t112: f64, t23912: f64, t26127: f64, t28012: f64, t28014: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t28792, t28795, t28821, t28826, t28830, t28834) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1712(t1530, t1649, t28, t5660, t191, t192, t6295, t1390, t6330, t1799, t1845, t6347);
        let (t28893, t28935, t28941) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1713(t5456, t576, t2031, t27956, t1860, t2032, t23963, t23995, t26016, t26911, t26920, t26936, t26948, t26954, t26960, t27937, t27961, t27966, t27972, t27976, t27979, t27982, t7026, t7428, t7432, t7435, t7782);
        let (t28942, t28943, t28951) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1714(t5, t109, t28941, t112, t23912, t26127, t28012, t28014);
    (t28792, t28795, t28821, t28826, t28830, t28834, t28893, t28935, t28942, t28943, t28951)
}
