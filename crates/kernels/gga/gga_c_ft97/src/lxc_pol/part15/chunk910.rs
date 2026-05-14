//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 910/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk910<F: Float>(t488: F, t86197: F, t86242: F, t86285: F, t86313: F, t4551: F, t38652: F, t110: F, t11863: F, t1871: F, t1901: F, t20098: F, t4458: F, t446: F, t447: F, t452: F, t4623: F, t47836: F, t47860: F, t60919: F, t83: F, t85546: F, t86010: F, t86193: F, t942: F, t979: F) -> (F, F, F) {
    let t86316 = t488 * (t86197 + t86242 + t86285 + t86313);
    let t86320 = t4551 * t4551;
    let t86321 = t38652 * t86320;
    let t86329 = -112.0 / 81.0 * t47836 + 112.0 / 81.0 * t47860 + 8.0 / 3.0 * t446 * t1871 * t110 * t20098 * t942 + 4.0 / 3.0 * t446 * t452 * t488 * t20098 * t979 + 4.0 / 3.0 * t446 * t447 * t4623 * t4458 + 2.0 / 3.0 * t446 * t447 * t110 * t85546 - 4.0 / 3.0 * t446 * t83 * t86010 - t446 * t83 * t86316 / 3.0 + 8.0 * t446 * t83 * t86321 - 8.0 / 27.0 * t60919 - 8.0 / 3.0 * t1901 * t11863 * t86193;
    (t86316, t86321, t86329)
}
