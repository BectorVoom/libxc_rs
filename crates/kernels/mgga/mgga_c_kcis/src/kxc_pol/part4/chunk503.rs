//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 503/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk503<F: Float>(t2339: F, t2342: F, t56: F, t649: F, t66: F, t45: F, t5: F, t103: F, t681: F, t52: F, t672: F, t678: F) -> (F, F, F, F, F, F) {
    let t2344 = F::cast_from(0.16081824322151104822e2_f64) * t2339 * t2342;
    let t2346 = t649 * t66 * t56;
    let t2349 = t45 * t5;
    let t2350 = t103 * t681;
    let t2353 = t672 * t52;
    let t2354 = F::new(1.0) / t2353;
    let t2355 = t678 * t678;
    (t2344, t2346, t2349, t2350, t2354, t2355)
}
