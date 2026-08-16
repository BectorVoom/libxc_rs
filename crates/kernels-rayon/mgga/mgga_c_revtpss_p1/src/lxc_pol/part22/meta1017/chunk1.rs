//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3517/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3517(t19477: f64, t3153: f64, t1011: f64, t15926: f64, t15950: f64, t16012: f64, t16045: f64, t16089: f64, t19705: f64, t19809: f64, t3092: f64, t3117: f64, t3241: f64, t42781: f64, t42785: f64, t4772: f64, t4873: f64, t4899: f64, t4900: f64, t4919: f64, t54261: f64, t54303: f64, t54306: f64, t63258: f64, t63283: f64, t63288: f64, t905: f64) -> (f64, f64) {
    let t66565 = t19477 * t3153;
    let t66591 = 0.30488190661738479624e-2_f64 * t54261 + 0.11433071498151929859e-2_f64 * t16089 * t3092 * t4772 * t905 * t4873 - 0.42874018118069736972e-3_f64 * t4899 * t3117 * t66565 * t4900 + 0.84689418504829110067e-4_f64 * t42781 + 0.6351706387862183255e-4_f64 * t42785 + 0.11433071498151929859e-2_f64 * t16089 * t3092 * t19705 * t15950 - 0.42874018118069736972e-3_f64 * t15926 * t16045 + t1011 * t4919 * t63258 / 108.0_f64 + t1011 * t4919 * t63283 / 216.0_f64 + 7.0_f64 / 648.0_f64 * t1011 * t16012 * t63288 + 4.0_f64 / 27.0_f64 * t3241 * t19809 + 2.0_f64 / 81.0_f64 * t54303 + t54306 / 72.0_f64;
    (t66565, t66591)
}
