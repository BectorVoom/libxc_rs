//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1377/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1377(t1041: f64, t10870: f64, t3048: f64, t3053: f64, t10478: f64, t3128: f64, t10472: f64, t1015: f64, t1030: f64, t3036: f64, t3033: f64, t248: f64, t3041: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10871 = t1041 * t10870;
    let t10873 = t3048 * t3053;
    let t10875 = t3128 * t10478;
    let t10876 = t10472 * t10875;
    let t10882 = t1015 * t10478;
    let t10883 = t10472 * t10882;
    let t10889 = t1030 * t3036;
    let t10890 = t1015 * t10889;
    let t10891 = t3033 * t10890;
    let t10895 = t248 * t3101 * t3041;
    (t10871, t10873, t10876, t10883, t10889, t10891, t10895)
}
