//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 947/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk947(t3436: f64, t57: f64, t875: f64, t3439: f64, t10978: f64, t10831: f64, t1102: f64, t1104: f64, t263: f64, t6876: f64, t2315: f64, t3438: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10979 = t57 * t3436;
    let t10980 = t10979 * t875;
    let t10981 = t10980 * t3439;
    let t10982 = t10978 * t10981;
    let t10983 = 0.43368970657079495312e-4_f64 * t10982;
    let t10990 = t1102 * t10831 * t1104;
    let t10991 = 0.14905073231436680509e-2_f64 * t10990;
    let t10992 = t263 * t6876;
    let t10993 = t3438 * t2315;
    (t10979, t10980, t10981, t10983, t10991, t10992, t10993)
}
