//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 586/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk586<F: Float>(t10215: F, t493: F, t492: F, t169: F, t172: F, t452: F, t1365: F, t7906: F, t6525: F, t10184: F, t10187: F, t10195: F, t10198: F, t105: F, t3341: F, t3359: F, t380: F, t419: F, t9151: F, t9207: F, t9210: F) -> (F, F, F) {
    let t10216 = t493 * t10215;
    let t10217 = t492 * t10216;
    let t10223 = t10215 * t169 * t172;
    let t10224 = t452 * t10223;
    let t10227 = t1365 * t7906;
    let t10228 = t6525 * t10227;
    let t10229 = F::cast_from(0.11856252764865062333e-2_f64) * t10228;
    let t10230 = t9151 + t10184 + t10187 + F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3341 - F::cast_from(0.37940008847568199465e-1_f64) * t380 * t3359 - F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3359 - t10195 - t10198 - t9207 + t9210 - F::cast_from(0.28455006635676149599e-1_f64) * t105 * t10217 + F::cast_from(0.28455006635676149599e-1_f64) * t419 * t3341 + F::cast_from(0.28455006635676149599e-1_f64) * t105 * t10224 + t10229;
    (t10216, t10223, t10230)
}
