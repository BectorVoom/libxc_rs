//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2175/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2175(t17841: f64, t340: f64, t343: f64, t974: f64, t135: f64, t5838: f64, t973: f64, t17801: f64, t17805: f64, t17809: f64, t17811: f64, t17814: f64, t17818: f64, t17821: f64, t17827: f64, t2960: f64, t2986: f64, t5839: f64, t5845: f64) -> (f64, f64, f64, f64) {
    let t17843 = t340 * t17841 * t343;
    let t17844 = t974 * t17843;
    let t17849 = t135 * t5838;
    let t17850 = t973 * t17849;
    let t17852 = -0.27777777777777777777e-3_f64 * t2986 * t17801 - 0.27777777777777777777e-3_f64 * t2986 * t17805 - 0.18518518518518518518e-3_f64 * t17809 + 0.16666666666666666666e-2_f64 * t2986 * t17811 - 0.11111111111111111111e-2_f64 * t2986 * t17814 + 0.55555555555555555554e-3_f64 * t2986 * t17818 - 0.55555555555555555554e-3_f64 * t2986 * t17821 + 0.22222222222222222222e-2_f64 * t2960 * t5845 - 0.27777777777777777777e-3_f64 * t17827 - 0.83333333333333333332e-3_f64 * t973 * t17844 + 0.22222222222222222222e-2_f64 * t2960 * t5839 - 0.27777777777777777777e-3_f64 * t17850;
    (t17843, t17844, t17849, t17852)
}
