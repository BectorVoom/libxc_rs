//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 586/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk586(t10215: f64, t493: f64, t492: f64, t169: f64, t172: f64, t452: f64, t1365: f64, t7906: f64, t6525: f64, t10184: f64, t10187: f64, t10195: f64, t10198: f64, t105: f64, t3341: f64, t3359: f64, t380: f64, t419: f64, t9151: f64, t9207: f64, t9210: f64) -> (f64, f64, f64) {
    let t10216 = t493 * t10215;
    let t10217 = t492 * t10216;
    let t10223 = t10215 * t169 * t172;
    let t10224 = t452 * t10223;
    let t10227 = t1365 * t7906;
    let t10228 = t6525 * t10227;
    let t10229 = 0.11856252764865062333e-2_f64 * t10228;
    let t10230 = t9151 + t10184 + t10187 + 0.37940008847568199465e-1_f64 * t380 * t3341 - 0.37940008847568199465e-1_f64 * t380 * t3359 - 0.28455006635676149599e-1_f64 * t419 * t3359 - t10195 - t10198 - t9207 + t9210 - 0.28455006635676149599e-1_f64 * t105 * t10217 + 0.28455006635676149599e-1_f64 * t419 * t3341 + 0.28455006635676149599e-1_f64 * t105 * t10224 + t10229;
    (t10216, t10223, t10230)
}
