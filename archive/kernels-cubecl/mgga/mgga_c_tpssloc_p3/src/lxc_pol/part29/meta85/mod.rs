//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta85 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk552;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk553;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk554;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk555;
use chunk4::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk556;
use chunk5::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk557;
use chunk6::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk558;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta85<F: Float>(t1734: F, t475: F, t1214: F, t248: F, t46: F, t480: F, t47: F, t479: F, t471: F, t1230: F, t1653: F, t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t467: F, t488: F, t466: F, t491: F, t1246: F, t493: F, t1244: F, t1729: F, t470: F, t494: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t1735 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk552::<F>(t1734, t475);
        let t1737 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk553::<F>(t1214, t1735, t248);
        let t1742 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk554::<F>(t46, t480, t47);
        let (t1743, t1744, t1748) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk555::<F>(t1742, t479, t471, t1230, t1653, t248);
        let t1751 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk556::<F>(t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t1744, t1748, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk557::<F>(t1751, t466, t1734, t491);
        let (t1756, t1758, t1760) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk558::<F>(t1246, t1755, t1751, t493, t1244, t1729, t470, t494);
    (t1735, t1737, t1742, t1743, t1744, t1748, t1751, t1752, t1755, t1756, t1758, t1760)
}
