//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 704/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk704(t158: f64, t3689: f64, t123: f64, t488: f64, t12000: f64, t169: f64, t172: f64, t452: f64, t10248: f64, t10251: f64, t10255: f64, t10259: f64, t10261: f64, t10264: f64, t10267: f64, t10271: f64, t10275: f64, t10278: f64, t105: f64, t1358: f64, t3692: f64, t3696: f64, t380: f64, t419: f64) -> (f64, f64, f64, f64) {
    let t12012 = t158 * t3689;
    let t12013 = t12012 * t123;
    let t12014 = t12013 * t488;
    let t12018 = t12000 * t169 * t172;
    let t12019 = t452 * t12018;
    let t12028 = -t10248 + t10251 - t10255 - t10259 + t10261 + t10264 + t10267 - t10271 - t10275 - 0.31616674039640166221e-2_f64 * t1358 * t12014 + 0.28455006635676149599e-1_f64 * t105 * t12019 - 0.37940008847568199465e-1_f64 * t380 * t3696 + 0.37940008847568199465e-1_f64 * t380 * t3692 - 0.28455006635676149599e-1_f64 * t419 * t3696 + t10278;
    (t12012, t12013, t12018, t12028)
}
