//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1346/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1346<F: Float>(t105900: F, t2983: F, t95292: F, t95362: F, t40465: F, t5900: F, t105578: F, t12590: F, t2101: F, t2992: F, t105882: F, t105884: F, t105888: F, t105891: F, t105895: F, t105899: F, t95330: F, t96126: F, t96127: F) -> (F, F, F, F, F) {
    let t105903 = t95292 * t105900 * t2983 * t95362;
    let t105905 = t40465 * t5900;
    let t105907 = t105578 * t105905 * t12590;
    let t105909 = t2101 * t5900;
    let t105912 = t95292 * t105909 * t2992 * t95362;
    let t105915 = -t105882 / 2.0 - 22.0 / 9.0 * t105884 - t105888 / 12.0 - 4.0 / 3.0 * t105891 - t105895 + t96126 + t96127 - t105899 - t105903 / 9.0 - 4.0 / 9.0 * t105907 + t105912 / 3.0 - 4.0 / 9.0 * t95330;
    (t105903, t105907, t105909, t105912, t105915)
}
