//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2730/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2730(t10985: f64, t15017: f64, t39557: f64, t39558: f64, t39562: f64, t39565: f64, t39567: f64, t39570: f64, t39573: f64, t40968: f64, t40970: f64, t40973: f64, t40978: f64, t50198: f64, t50201: f64, t50205: f64, t50209: f64) -> f64 {
    let t50214 = t15017 * t10985;
    let t50216 = t39557 - 0.13878983423218070567e-1_f64 * t39558 - 0.19514881078765566037e-2_f64 * t39562 + 0.39029762157531132075e-2_f64 * t39565 - 0.39029762157531132075e-1_f64 * t39567 + 0.16463622957338778996e-1_f64 * t39570 - 0.39029762157531132075e-2_f64 * t39573 + 0.16463622957338778996e-1_f64 * t50198 + 0.29272321618148349057e-1_f64 * t50201 + 0.16463622957338778996e-1_f64 * t40968 - 0.30356481678079769392e-1_f64 * t50205 + 0.58544643236296698113e-1_f64 * t50209 - 0.7805952431506226415e-2_f64 * t40970 + 0.54878743191129263322e-2_f64 * t40973 - 0.58911598146606471822e-3_f64 * t40978 - 0.46263278077393568556e-2_f64 * t50214;
    t50216
}
