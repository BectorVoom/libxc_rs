//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 504/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk504<F: Float>(t668: F, t870: F, t505: F, t875: F, t2881: F, t2409: F, t319: F, t835: F, t1901: F, t193: F, t2751: F, t2803: F, t2807: F, t2811: F, t2816: F, t2817: F, t2819: F, t2834: F, t2839: F, t2846: F, t2850: F, t2854: F, t2859: F, t2864: F, t2869: F, t2872: F, t2878: F, t446: F, t89: F) -> (F, F, F, F, F, F) {
    let t2882 = t870 * t668;
    let t2883 = t505 * t875;
    let t2884 = t2882 * t2883;
    let t2885 = t2881 * t2884;
    let t2889 = t835 * t319 * t2409;
    let t2892 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2751 - t446 * t2803 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2807 - t446 * t2811 / F::cast_from(3.0_f64) + t2816 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2817 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2819 + t89 * t193 * t2834 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t2839 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2846 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t2850 - t446 * t2854 / F::cast_from(9.0_f64) - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t446 * t2859 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2864 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t2869 + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t2872 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t2878 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t2885 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t2889;
    (t2882, t2883, t2884, t2885, t2889, t2892)
}
