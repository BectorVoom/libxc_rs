//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta563 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2067;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2068;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta563(t10471: f64, t42332: f64, t10875: f64, t10468: f64, t191: f64, t349: f64, t68: f64) -> (f64, f64, f64, f64, f64) {
        let (t42333, t42334, t42339, t42340) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2067(t10471, t42332, t10875, t10468, t191, t349);
        let t42341 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2068(t10471, t68);
    (t42333, t42334, t42339, t42340, t42341)
}
