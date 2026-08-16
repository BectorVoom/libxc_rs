//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2118/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2118(t10471: f64, t42332: f64, t10875: f64, t10468: f64, t191: f64, t349: f64) -> (f64, f64, f64, f64) {
    let t42333 = t42332 * t10471;
    let t42334 = t42333 * t10875;
    let t42339 = 1.0_f64 / t10468 / t191;
    let t42340 = t349 * t42339;
    (t42333, t42334, t42339, t42340)
}
