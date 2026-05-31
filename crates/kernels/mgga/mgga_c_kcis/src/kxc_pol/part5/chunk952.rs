//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 952/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk952<F: Float>(t180: F, t9314: F, t160: F, t167: F, t1071: F, t253: F, t2843: F, t329: F, t2820: F, t2840: F, t86: F, t3225: F, sigma0: F) -> (F, F, F, F, F, F) {
    let t9315 = t180 * t9314;
    let t9323 = t167 * t160;
    let t9368 = F::cast_from(1.0_f64) / t253 / t1071;
    let t9372 = F::cast_from(1.0_f64) / t2843 / t329;
    let t9386 = t86 * t2820 * t2840;
    let t9409 = t3225 * sigma0;
    (t9315, t9323, t9368, t9372, t9386, t9409)
}
