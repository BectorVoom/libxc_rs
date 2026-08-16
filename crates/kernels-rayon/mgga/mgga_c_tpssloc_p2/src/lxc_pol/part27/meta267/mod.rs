//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta267 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1280;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1281;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1282;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1283;
use chunk4::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1284;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta267(t1603: f64, t1945: f64, t1409: f64, t3: f64, t1933: f64, t1597: f64, t343: f64, t6734: f64, t1615: f64, t68: f64, t360: f64, t6744: f64, t1611: f64, t1941: f64, t1607: f64, t1618: f64, t1622: f64, t1935: f64, t1937: f64, t378: f64, t6716: f64, t6717: f64, t6728: f64, t6742: f64, t6755: f64, t6763: f64, t6765: f64, t349: f64, t1634: f64, t1955: f64, t3174: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t7569, t7573, t7574, t7577) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1280(t1603, t1945, t1409, t3, t1933, t1597, t343);
        let t7578 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1281(t6734, t7577);
        let (t7581, t7582, t7583) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1282(t1615, t68, t360, t6744);
        let (t7586, t7593) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1283(t1611, t1941, t1607, t1618, t1622, t1935, t1937, t378, t6716, t6717, t6728, t6742, t6755, t6763, t6765, t7574, t7578, t7583);
        let (t7594, t7600) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1284(t349, t7593, t1634, t1955, t3174);
    (t7569, t7573, t7574, t7577, t7578, t7581, t7582, t7583, t7586, t7593, t7594, t7600)
}
