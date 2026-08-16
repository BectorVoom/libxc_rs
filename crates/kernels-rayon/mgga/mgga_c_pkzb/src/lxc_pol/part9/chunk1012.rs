//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1012/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1012(t2198: f64, t8205: f64, t6199: f64, t1189: f64, t2256: f64, t3030: f64, t832: f64, t853: f64, t2235: f64, t3033: f64, t1171: f64, t2239: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8206 = t8205 * t2198;
    let t8208 = 0.51726012919273400301e3_f64 * t6199 * t8206;
    let t8211 = t1189 * t2256;
    let t8214 = t3030 * t832;
    let t8216 = 2.0_f64 * t8214 * t853;
    let t8218 = 1.0_f64 * t3033 * t2235;
    let t8219 = t1171 * t2239;
    (t8206, t8208, t8211, t8214, t8216, t8218, t8219)
}
