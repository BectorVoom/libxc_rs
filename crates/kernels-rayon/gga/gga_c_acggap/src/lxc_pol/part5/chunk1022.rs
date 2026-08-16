//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1022/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1022(t1162: f64, t1535: f64, t17386: f64, t4393: f64, t4396: f64, t1418: f64, t3670: f64, t1347: f64, t1429: f64, t3237: f64, t5255: f64, t997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17388 = t17386 * t1162 * t1535;
    let t17390 = t4396 * t4393;
    let t17392 = t3670 * t1418;
    let t17395 = t3670 * t1347;
    let t17397 = t3237 * t1429;
    let t17399 = t997 * t5255;
    (t17388, t17390, t17392, t17395, t17397, t17399)
}
