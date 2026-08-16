//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1111/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1111(t12254: f64, t2863: f64, t9493: f64, t1519: f64, t2911: f64, t1543: f64, t2975: f64, t1053: f64, t4117: f64, t1523: f64, t2954: f64, t1063: f64, t12000: f64, t12219: f64, t12222: f64, t12243: f64, t12246: f64, t12250: f64, t12253: f64, t2950: f64, t2955: f64, t2958: f64, t2999: f64, t4120: f64, t9380: f64) -> (f64, f64, f64) {
    let t12255 = t12254 * t2863;
    let t12257 = 0.51726012919273400301e3_f64 * t9493 * t12255;
    let t12258 = t1519 * t2863;
    let t12260 = 6.0_f64 * t2911 * t12258;
    let t12261 = t1543 * t2975;
    let t12264 = t4117 * t1053;
    let t12269 = t1523 * t2954;
    let t12273 = 0.10254018858216406658e4_f64 * t9380 * t12219 + 6.0_f64 * t2955 * t12222 + t12243 + t12246 - t12250 - t12253 - t12257 - t12260 + 0.35089341735807877242e1_f64 * t2999 * t12261 + 2.0_f64 * t12264 * t1063 + 1.0_f64 * t4120 * t2950 + 0.32163958997385070134e2_f64 * t12269 * t2958 - 0.19751673498613801407e-1_f64 * t12000;
    (t12257, t12260, t12273)
}
