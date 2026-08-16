//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 674/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk674(t10887: f64, t2021: f64, t2975: f64, t7372: f64, t1: f64, t10686: f64, t787: f64, t2465: f64, t2949: f64, t2464: f64, t825: f64, t8516: f64, t959: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10888 = 0.14896037479937677779e-1_f64 * t10887;
    let t10889 = t2021 * t2975;
    let t10890 = t10889 * t7372;
    let t10891 = 0.14896037479937677779e-1_f64 * t10890;
    let t10892 = t10686 * t1;
    let t10893 = t787 * t10892;
    let t10896 = t2465 * t2949;
    let t10897 = t2464 * t10896;
    let t10898 = t825 * t10897;
    let t10899 = 0.42603251059911944084e-1_f64 * t10898;
    let t10900 = t8516 * t959;
    (t10888, t10891, t10892, t10893, t10899, t10900)
}
