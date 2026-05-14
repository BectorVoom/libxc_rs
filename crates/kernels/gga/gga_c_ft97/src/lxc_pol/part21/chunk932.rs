//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 932/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk932<F: Float>(t29569: F, t369: F, t108: F, t28: F, t25861: F, t925: F, t1564: F, t26117: F, t4462: F, t5502: F, t4454: F, t7793: F, t4458: F, t1332: F, t4551: F, t8418: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29570 = t369 * t29569;
    let t29571 = t29570 * t108;
    let t29572 = t28 * t29571;
    let t29577 = t25861 * t925;
    let t29578 = t1564 * t29577;
    let t29582 = t1564 * t26117 * t925;
    let t29586 = t1564 * t5502 * t4462;
    let t29590 = t7793 * t5502 * t4454;
    let t29594 = t1564 * t5502 * t4458;
    let t29599 = t1332 * t4551;
    let t29600 = t8418 * t29599;
    (t29570, t29571, t29572, t29578, t29582, t29586, t29590, t29594, t29599, t29600)
}
