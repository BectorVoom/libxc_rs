//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1002/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1002(t10029: f64, t13609: f64, t187: f64, t12908: f64, t10031: f64, t10033: f64, t12920: f64, t12922: f64, t10038: f64, t10042: f64, t12913: f64, t12915: f64, t12918: f64, t12924: f64, t7979: f64, t7988: f64, t7992: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13810 = 0.5848223622634646207e0_f64 * t10029;
    let t13812 = 0.19751673498613801407e-1_f64 * t13609 * t187;
    let t13813 = 24.0_f64 * t12908;
    let t13814 = 32.0_f64 * t10031;
    let t13815 = 20.0_f64 * t10033;
    let t13816 = 0.34631718211362927517e2_f64 * t12920;
    let t13817 = 0.11696447245269292414e1_f64 * t12922;
    let t13818 = -t13810 + t7979 + t13812 - t13813 - t12913 - t12915 + t12918 + t13814 + t13815 - t13816 - t13817 - t10038 - t12924 - t10042 + t7988 + t7992;
    (t13810, t13812, t13813, t13814, t13815, t13816, t13817, t13818)
}
