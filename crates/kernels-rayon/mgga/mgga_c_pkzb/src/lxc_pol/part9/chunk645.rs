//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 645/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk645(t2900: f64, t2901: f64, t302: f64, t1125: f64, t2099: f64, t757: f64, t1137: f64, t2106: f64, t2105: f64, t1120: f64, t1126: f64, t2047: f64, t2051: f64, t2060: f64, t2096: f64, t2104: f64, t276: f64, t2884: f64, t2887: f64, t2891: f64, t2895: f64, t2899: f64, t735: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2902 = t2900 * t2901;
    let t2903 = t302 * t2902;
    let t2908 = t2099 * t1125;
    let t2909 = t757 * t2908;
    let t2911 = t1137 * t2106;
    let t2912 = t2105 * t2911;
    let t2915 = -t2060 / 108.0_f64 - t2047 - t2051 / 288.0_f64 + t735 * t1120 / 36.0_f64 - t2884 / 288.0_f64 + t2887 * t2891 / 48.0_f64 - t276 * t2895 / 96.0_f64 + 0.42874018118069736972e-3_f64 * t2899 * t2903 - 0.11433071498151929859e-2_f64 * t2096 * t1126 + 0.14291339372689912324e-3_f64 * t2909 - 0.42874018118069736972e-3_f64 * t2104 * t2912;
    (t2902, t2903, t2908, t2911, t2912, t2915)
}
