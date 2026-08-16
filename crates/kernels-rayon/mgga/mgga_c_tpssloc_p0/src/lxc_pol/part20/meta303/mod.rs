//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta303 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1544;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1545;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1546;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta303(t11177: f64, t300: f64, t1098: f64, t3256: f64, t1119: f64, t3259: f64, t3308: f64, t1094: f64, t3312: f64, t3316: f64, t3311: f64, t419: f64, t409: f64, t1117: f64, t3265: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11179, t11180, t11182, t11184, t11185) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1544(t11177, t300, t1098, t3256, t1119, t3259, t3308, t1094, t3312);
        let (t11187, t11189, t11190) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1545(t11185, t3316, t3311, t419, t409);
        let t11191 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1546(t1117, t3265);
    (t11179, t11180, t11182, t11184, t11185, t11187, t11189, t11190, t11191)
}
