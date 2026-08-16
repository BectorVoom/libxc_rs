//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 381/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk381(t695: f64, t702: f64, t1060: f64, t1919: f64, t1860: f64, t673: f64, t140: f64, t1470: f64, t1883: f64, t1888: f64, t1909: f64, t1918: f64, t479: f64, t709: f64, t725: f64) -> (f64, f64, f64, f64) {
    let t1920 = t702 * t695;
    let t1922 = t1919 * t1920 * t1060;
    let t1925 = t673 * t1860;
    let t1929 = 0.619125e-2_f64 * t1909 * t709 + 0.9286875e-2_f64 * t725 * t1883 - 0.619125e-2_f64 * t725 * t1888 - t1918 - 0.26531111111111111111e-1_f64 * t1470 * t1922 - 0.39796666666666666666e-1_f64 * t140 * t479 * t1925;
    (t1920, t1922, t1925, t1929)
}
