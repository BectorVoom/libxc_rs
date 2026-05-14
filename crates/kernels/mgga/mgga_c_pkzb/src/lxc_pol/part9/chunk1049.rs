//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1049/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1049<F: Float>(t1542: F, t2605: F, t1020: F, t1816: F, t16613: F, t16615: F, t16617: F, t16619: F, t16621: F, t1009: F, t4803: F, t5142: F, t1639: F, t7035: F, t2706: F, t5165: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19742 = t1542 * t2605;
    let t19743 = 60.0 * t19742;
    let t19744 = t1020 * t1816;
    let t19748 = 240.0 * t16613;
    let t19749 = 0.31168546390226634765e3 * t16615;
    let t19750 = 0.30762056574649219973e4 * t16617;
    let t19751 = 36.0 * t16619;
    let t19752 = 96.0 * t16621;
    let t19754 = t4803 * t1009;
    let t19755 = 24.0 * t19754;
    let t19756 = t5142 * t1009;
    let t19757 = 144.0 * t19756;
    let t19758 = t7035 * t1639;
    let t19759 = 0.35089341735807877242e1 * t19758;
    let t19766 = t2706 * t5165;
    (t19743, t19744, t19748, t19749, t19750, t19751, t19752, t19755, t19757, t19759, t19766)
}
