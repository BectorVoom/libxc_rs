//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1093/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1093(t15074: f64, t15075: f64, t345: f64, t242: f64, t947: f64, t2724: f64, t3949: f64, t3932: f64, t3931: f64, t8561: f64, t948: f64, t14920: f64) -> (f64, f64, f64, f64) {
    let t15076 = t15074 + t15075;
    let t15077 = t15076 * t345;
    let t15079 = t242 * t947 * t15077;
    let t15082 = t2724 * t3949;
    let t15083 = t3932 * t15082;
    let t15084 = t3931 * t15083;
    let t15087 = t8561 * t948;
    let t15088 = t14920 * t15087;
    (t15076, t15079, t15084, t15088)
}
