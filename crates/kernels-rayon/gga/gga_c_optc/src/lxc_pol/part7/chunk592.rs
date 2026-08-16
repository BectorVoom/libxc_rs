//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 592/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk592(t1094: f64, t2916: f64, t2917: f64, t1102: f64, t2843: f64, t2845: f64, t2852: f64, t2858: f64, t2862: f64, t1062: f64, t1066: f64, t1065: f64, t398: f64) -> (f64, f64, f64, f64, f64) {
    let t2919 = t2916 * t2917 * t1094;
    let t2921 = 0.11696446794910408142e1_f64 * t1102 * t2919;
    let t2922 = 0.22831111111111111111e-1_f64 * t2843;
    let t2927 = t2922 + 0.11415555555555555555e-1_f64 * t2845 - 0.11415555555555555555e-1_f64 * t2852 + 0.34246666666666666666e-1_f64 * t2858 - 0.17123333333333333333e-1_f64 * t2862;
    let t2930 = t1062 * t1066;
    let t2933 = t1065 * t398;
    (t2919, t2921, t2927, t2930, t2933)
}
