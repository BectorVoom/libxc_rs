//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1215/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1215<F: Float>(t30760: F, t694: F, t5005: F, t703: F, t5049: F, t52324: F, t688: F, t5001: F, t6022: F, t27604: F, t5014: F, t24378: F, t30667: F, t6034: F, t24276: F, t30594: F, t96419: F) -> (F, F, F, F, F, F, F, F) {
    let t122858 = t694 * t30760;
    let t122869 = t703 * t5005;
    let t122874 = t703 * t5049;
    let t122889 = t52324 * t688;
    let t122895 = t6022 * t5001;
    let t122899 = t27604 * t5014;
    let t122904 = t6034 * t24378 * t30667;
    let t122909 = t24276 * t96419 * t30594;
    (t122858, t122869, t122874, t122889, t122895, t122899, t122904, t122909)
}
