//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1293/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1293(t3236: f64, t5819: f64, t1012: f64, t1015: f64, t5825: f64, t3253: f64, t1011: f64, t1025: f64, t1665: f64, t3082: f64, t3091: f64, t3115: f64, t3127: f64, t4792: f64, t4818: f64, t4821: f64, t4858: f64, t6263: f64, t6268: f64, t6273: f64, t6278: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6284 = t3236 * t5819;
    let t6285 = t1012 * t6284;
    let t6288 = t1015 * t5825;
    let t6289 = t1012 * t6288;
    let t6292 = t3253 * t5819;
    let t6293 = t1012 * t6292;
    let t6298 = -t3082 - 0.28582678745379824648e-3_f64 * t3127 * t6263 + 0.28582678745379824648e-3_f64 * t3091 * t6268 - 0.42874018118069736972e-3_f64 * t3115 * t6273 - 0.21437009059034868486e-3_f64 * t1025 * t6278 - 0.42874018118069736972e-3_f64 * t4858 * t1665 + 0.28582678745379824648e-3_f64 * t4792 - t1011 * t6285 / 144.0_f64 + t1011 * t6289 / 288.0_f64 + t1011 * t6293 / 216.0_f64 + 0.19055119163586549765e-3_f64 * t4818 + 0.28582678745379824648e-3_f64 * t4821;
    (t6284, t6285, t6288, t6289, t6292, t6293, t6298)
}
