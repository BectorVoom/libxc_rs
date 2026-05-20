//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1048/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1048<F: Float>(t3236: F, t5819: F, t1012: F, t1015: F, t5825: F, t3253: F, t1011: F, t1025: F, t1665: F, t3082: F, t3091: F, t3115: F, t3127: F, t4792: F, t4818: F, t4821: F, t4858: F, t6263: F, t6268: F, t6273: F, t6278: F) -> (F, F, F, F, F, F, F) {
    let t6284 = t3236 * t5819;
    let t6285 = t1012 * t6284;
    let t6288 = t1015 * t5825;
    let t6289 = t1012 * t6288;
    let t6292 = t3253 * t5819;
    let t6293 = t1012 * t6292;
    let t6298 = -t3082 - F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t6263 + F::cast_from(0.28582678745379824648e-3_f64) * t3091 * t6268 - F::cast_from(0.42874018118069736972e-3_f64) * t3115 * t6273 - F::cast_from(0.21437009059034868486e-3_f64) * t1025 * t6278 - F::cast_from(0.42874018118069736972e-3_f64) * t4858 * t1665 + F::cast_from(0.28582678745379824648e-3_f64) * t4792 - t1011 * t6285 / F::new(144.0) + t1011 * t6289 / F::new(288.0) + t1011 * t6293 / F::new(216.0) + F::cast_from(0.19055119163586549765e-3_f64) * t4818 + F::cast_from(0.28582678745379824648e-3_f64) * t4821;
    (t6284, t6285, t6288, t6289, t6292, t6293, t6298)
}
