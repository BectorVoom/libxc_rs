//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 494/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk494(t668: f64, t870: f64, t505: f64, t875: f64, t2881: f64, t2409: f64, t319: f64, t835: f64, t1901: f64, t193: f64, t2751: f64, t2803: f64, t2807: f64, t2811: f64, t2816: f64, t2817: f64, t2819: f64, t2834: f64, t2839: f64, t2846: f64, t2850: f64, t2854: f64, t2859: f64, t2864: f64, t2869: f64, t2872: f64, t2878: f64, t446: f64, t89: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2882 = t870 * t668;
    let t2883 = t505 * t875;
    let t2884 = t2882 * t2883;
    let t2885 = t2881 * t2884;
    let t2889 = t835 * t319 * t2409;
    let t2892 = -2.0_f64 / 3.0_f64 * t446 * t2751 - t446 * t2803 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t446 * t2807 - t446 * t2811 / 3.0_f64 + t2816 + 2.0_f64 / 9.0_f64 * t2817 + 2.0_f64 / 9.0_f64 * t2819 + t89 * t193 * t2834 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t2839 + 2.0_f64 / 3.0_f64 * t446 * t2846 - 2.0_f64 / 9.0_f64 * t446 * t2850 - t446 * t2854 / 9.0_f64 - 2.0_f64 / 27.0_f64 * t446 * t2859 + 2.0_f64 / 3.0_f64 * t446 * t2864 + 2.0_f64 / 3.0_f64 * t446 * t2869 + 2.0_f64 / 27.0_f64 * t2872 + 2.0_f64 / 9.0_f64 * t1901 * t2878 + 2.0_f64 / 9.0_f64 * t1901 * t2885 + 2.0_f64 / 9.0_f64 * t446 * t2889;
    (t2882, t2883, t2884, t2885, t2889, t2892)
}
