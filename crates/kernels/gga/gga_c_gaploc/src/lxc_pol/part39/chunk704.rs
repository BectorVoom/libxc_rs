//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 704/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk704<F: Float>(t158: F, t3689: F, t123: F, t488: F, t12000: F, t169: F, t172: F, t452: F, t10248: F, t10251: F, t10255: F, t10259: F, t10261: F, t10264: F, t10267: F, t10271: F, t10275: F, t10278: F, t105: F, t1358: F, t3692: F, t3696: F, t380: F, t419: F) -> (F, F, F, F) {
    let t12012 = t158 * t3689;
    let t12013 = t12012 * t123;
    let t12014 = t12013 * t488;
    let t12018 = t12000 * t169 * t172;
    let t12019 = t452 * t12018;
    let t12028 = -t10248 + t10251 - t10255 - t10259 + t10261 + t10264 + t10267 - t10271 - t10275 - F::new(0.31616674039640166221e-2) * t1358 * t12014 + F::new(0.28455006635676149599e-1) * t105 * t12019 - F::new(0.37940008847568199465e-1) * t380 * t3696 + F::new(0.37940008847568199465e-1) * t380 * t3692 - F::new(0.28455006635676149599e-1) * t419 * t3696 + t10278;
    (t12012, t12013, t12018, t12028)
}
