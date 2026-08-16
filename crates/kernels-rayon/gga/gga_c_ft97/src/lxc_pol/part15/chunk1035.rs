//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1035/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1035(t488: f64, t86197: f64, t86242: f64, t86285: f64, t86313: f64, t4551: f64, t38652: f64, t110: f64, t11863: f64, t1871: f64, t1901: f64, t20098: f64, t4458: f64, t446: f64, t447: f64, t452: f64, t4623: f64, t47836: f64, t47860: f64, t60919: f64, t83: f64, t85546: f64, t86010: f64, t86193: f64, t942: f64, t979: f64) -> (f64, f64, f64) {
    let t86316 = t488 * (t86197 + t86242 + t86285 + t86313);
    let t86320 = t4551 * t4551;
    let t86321 = t38652 * t86320;
    let t86329 = -112.0_f64 / 81.0_f64 * t47836 + 112.0_f64 / 81.0_f64 * t47860 + 8.0_f64 / 3.0_f64 * t446 * t1871 * t110 * t20098 * t942 + 4.0_f64 / 3.0_f64 * t446 * t452 * t488 * t20098 * t979 + 4.0_f64 / 3.0_f64 * t446 * t447 * t4623 * t4458 + 2.0_f64 / 3.0_f64 * t446 * t447 * t110 * t85546 - 4.0_f64 / 3.0_f64 * t446 * t83 * t86010 - t446 * t83 * t86316 / 3.0_f64 + 8.0_f64 * t446 * t83 * t86321 - 8.0_f64 / 27.0_f64 * t60919 - 8.0_f64 / 3.0_f64 * t1901 * t11863 * t86193;
    (t86316, t86321, t86329)
}
