//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1038/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1038<F: Float>(t5838: F, t92834: F, t5829: F, t5830: F, t92557: F, t5813: F, t5814: F, t5821: F, t92574: F, t3392: F, t94375: F, t8811: F, t93178: F, t135: F, t5555: F, t5824: F) -> (F, F, F, F, F, F, F, F) {
    let t94620 = t5838 * t92834;
    let t94722 = t5829 * t92557 * t5830;
    let t94821 = t5813 * t92557 * t5814;
    let t94823 = t5821 * t92574;
    let t94838 = t3392 * t94375;
    let t94891 = t8811 * t93178;
    let t94892 = t5555 * t135;
    let t94932 = t5824 * t92574;
    (t94620, t94722, t94821, t94823, t94838, t94891, t94892, t94932)
}
