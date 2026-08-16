//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 541/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk541(t2880: f64, t932: f64, t922: f64, t302: f64, t310: f64, t2862: f64, t2764: f64, t2766: f64, t2773: f64, t2778: f64, t2782: f64, t324: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2881 = t2880 * t932;
    let t2884 = t922 * t922;
    let t2885 = 1.0_f64 / t2884;
    let t2886 = t302 * t2885;
    let t2887 = t310 * t310;
    let t2888 = 1.0_f64 / t2887;
    let t2889 = t2862 * t2888;
    let t2892 = 0.12361111111111111111e-1_f64 * t2764;
    let t2897 = t2892 + 0.61805555555555555556e-2_f64 * t2766 - 0.61805555555555555555e-2_f64 * t2773 + 0.18541666666666666667e-1_f64 * t2778 - 0.92708333333333333333e-2_f64 * t2782;
    let t2898 = t2897 * t324;
    (t2881, t2884, t2885, t2886, t2887, t2888, t2889, t2897, t2898)
}
