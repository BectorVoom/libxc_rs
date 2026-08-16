//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 825/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk825(t1464: f64, t15960: f64, t3738: f64, t5876: f64, t13396: f64, t1392: f64, t86: f64, t5782: f64, t4177: f64, t5752: f64, t1394: f64, t2001: f64, t4124: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15961 = t1464 * t15960;
    let t15963 = t3738 * t5876;
    let t15964 = t1464 * t15963;
    let t15967 = t86 * t13396 * t1392;
    let t15968 = t15967 * t5782;
    let t15970 = t5752 * t4177;
    let t15971 = t1394 * t15970;
    let t15973 = t2001 * t4124;
    (t15961, t15964, t15967, t15968, t15971, t15973)
}
