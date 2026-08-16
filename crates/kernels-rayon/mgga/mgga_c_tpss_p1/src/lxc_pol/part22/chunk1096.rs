//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1096/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1096(t11999: f64, t434: f64, t294: f64, t3017: f64, t4192: f64, t3013: f64, t3009: f64, t4202: f64, t4155: f64, t1091: f64, t3154: f64, t4325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12000 = t11999 * t434;
    let t12002 = 0.19751673498613801407e-1_f64 * t294 * t12000;
    let t12004 = 0.5848223622634646207e0_f64 * t4192 * t3017;
    let t12006 = 0.11696447245269292414e1_f64 * t4192 * t3013;
    let t12008 = 0.11696447245269292414e1_f64 * t3009 * t4202;
    let t12009 = t294 * t4155;
    let t12011 = 0.11696447245269292414e1_f64 * t12009 * t1091;
    let t12012 = t4325 * t3154;
    (t12000, t12002, t12004, t12006, t12008, t12011, t12012)
}
