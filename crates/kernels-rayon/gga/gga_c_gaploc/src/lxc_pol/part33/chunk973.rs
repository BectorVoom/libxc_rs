//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 973/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk973(t10882: f64, t2464: f64, t2684: f64, t787: f64, t8788: f64, t9824: f64, t2021: f64, t2975: f64, t7372: f64, t2465: f64, t2949: f64, t825: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t10883 = t2464 * t10882;
    let t10884 = t2684 * t10883;
    let t10885 = 0.42603251059911944084e-1_f64 * t10884;
    let t10886 = t787 * t8788;
    let t10887 = t10886 * t9824;
    let t10888 = 0.14896037479937677779e-1_f64 * t10887;
    let t10889 = t2021 * t2975;
    let t10890 = t10889 * t7372;
    let t10891 = 0.14896037479937677779e-1_f64 * t10890;
    let t10896 = t2465 * t2949;
    let t10897 = t2464 * t10896;
    let t10898 = t825 * t10897;
    (t10883, t10885, t10886, t10888, t10889, t10891, t10896, t10897, t10898)
}
