//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 996/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk996<F: Float>(t231: F, t5009: F, t14: F, t30674: F, t27695: F, t6758: F, t4977: F, t679: F, t200: F, t6014: F, t2378: F, t4939: F, t24385: F, t227: F, t5001: F, t52: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30676 = t231 * t5009;
    let t30677 = t30674 * t14 * t30676;
    let t30680 = t27695 * t6758;
    let t30683 = t679 * t4977;
    let t30684 = t30683 * t200;
    let t30685 = t6014 * t30684;
    let t30688 = t2378 * t4939;
    let t30689 = t30688 * t200;
    let t30690 = t24385 * t30689;
    let t30696 = t52 * t227 * t5001;
    (t30676, t30677, t30680, t30683, t30684, t30685, t30688, t30689, t30690, t30696)
}
