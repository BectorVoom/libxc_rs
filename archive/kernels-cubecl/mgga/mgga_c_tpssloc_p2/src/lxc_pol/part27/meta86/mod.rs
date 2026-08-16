//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta86 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk559;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk560;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk561;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta86<F: Float>(t1742: F, t479: F, t471: F, t1230: F, t1653: F, t248: F, t1174: F, t1195: F, t1213: F, t1224: F, t1227: F, t1706: F, t1726: F, t1731: F, t1737: F, t467: F, t488: F, t466: F, t1734: F, t491: F, t1246: F, t493: F, t1244: F, t1729: F, t470: F, t494: F, t1241: F, t1238: F, t1721: F, t498: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t1743, t1744, t1748, t1751) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk559::<F>(t1742, t479, t471, t1230, t1653, t248, t1174, t1195, t1213, t1224, t1227, t1706, t1726, t1731, t1737, t467, t488);
        let (t1752, t1755) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk560::<F>(t1751, t466, t1734, t491);
        let (t1756, t1758, t1760, t1761, t1763) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk561::<F>(t1246, t1755, t1751, t493, t1244, t1729, t470, t494, t1241, t1238, t1721, t1752, t498);
    (t1743, t1744, t1748, t1751, t1752, t1755, t1756, t1758, t1760, t1761, t1763)
}
