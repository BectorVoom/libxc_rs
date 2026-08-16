//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 865/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk865(t18648: f64, t9714: f64, t26: f64, t18657: f64, t2970: f64, t4714: f64, t18685: f64, t939: f64, t18570: f64, t945: f64, t18574: f64, t18677: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18908 = t9714 * t18648;
    let t18909 = t26 * t18908;
    let t18911 = t2970 * t18657;
    let t18912 = t4714 * t18911;
    let t18920 = t939 * t18685;
    let t18923 = t945 * t18570;
    let t18924 = t26 * t18923;
    let t18926 = t945 * t18574;
    let t18927 = t4714 * t18926;
    let t18929 = t945 * t18677;
    (t18909, t18912, t18920, t18924, t18927, t18929)
}
