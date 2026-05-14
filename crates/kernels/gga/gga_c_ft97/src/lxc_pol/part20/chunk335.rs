//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 335/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk335<F: Float>(t2882: F, t2883: F, t2881: F, t2409: F, t319: F, t835: F, t1901: F, t193: F, t2751: F, t2803: F, t2807: F, t2811: F, t2816: F, t2817: F, t2819: F, t2834: F, t2839: F, t2846: F, t2850: F, t2854: F, t2859: F, t2864: F, t2869: F, t2872: F, t2878: F, t446: F, t89: F) -> (F, F, F, F) {
    let t2884 = t2882 * t2883;
    let t2885 = t2881 * t2884;
    let t2889 = t835 * t319 * t2409;
    let t2892 = -2.0 / 3.0 * t446 * t2751 - t446 * t2803 / 3.0 - 2.0 / 3.0 * t446 * t2807 - t446 * t2811 / 3.0 + t2816 + 2.0 / 9.0 * t2817 + 2.0 / 9.0 * t2819 + t89 * t193 * t2834 / 3.0 - 2.0 / 9.0 * t2839 + 2.0 / 3.0 * t446 * t2846 - 2.0 / 9.0 * t446 * t2850 - t446 * t2854 / 9.0 - 2.0 / 27.0 * t446 * t2859 + 2.0 / 3.0 * t446 * t2864 + 2.0 / 3.0 * t446 * t2869 + 2.0 / 27.0 * t2872 + 2.0 / 9.0 * t1901 * t2878 + 2.0 / 9.0 * t1901 * t2885 + 2.0 / 9.0 * t446 * t2889;
    (t2884, t2885, t2889, t2892)
}
