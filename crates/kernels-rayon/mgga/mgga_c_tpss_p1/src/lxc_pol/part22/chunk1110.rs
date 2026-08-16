//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1110/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1110(t12241: f64, t408: f64, t1505: f64, t2861: f64, t2864: f64, t2913: f64, t4104: f64, t1042: f64, t2911: f64, t2905: f64, t4108: f64, t1518: f64, t9495: f64) -> (f64, f64, f64, f64, f64) {
    let t12243 = 0.621814e-1_f64 * t12241 * t408;
    let t12244 = t1505 * t2861;
    let t12246 = 2.0_f64 * t12244 * t2864;
    let t12247 = t4104 * t2913;
    let t12248 = t12247 * t1042;
    let t12250 = 0.32163958997385070134e2_f64 * t2911 * t12248;
    let t12251 = t4108 * t2905;
    let t12253 = 0.16081979498692535067e2_f64 * t2911 * t12251;
    let t12254 = t1518 * t9495;
    (t12243, t12246, t12250, t12253, t12254)
}
