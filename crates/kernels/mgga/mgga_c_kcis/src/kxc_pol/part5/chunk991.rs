//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 991/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk991<F: Float>(t1460: F, t3245: F, t10470: F, t558: F, t530: F, t64: F, t555: F, t491: F, t1502: F, t4188: F, t1504: F, t561: F) -> (F, F, F, F, F, F, F) {
    let t12303 = t3245 * t1460;
    let t12305 = t10470 * t558;
    let t12306 = F::cast_from(0.73697530864197530862e-3_f64) * t12305;
    let t12319 = t64 * t530;
    let t12321 = F::new(1.0) / t555 / t12319;
    let t12322 = t491 * t12321;
    let t12338 = t1502 * t4188;
    let t12343 = t1504 * t1504;
    let t12344 = F::new(1.0) / t12343;
    let t12345 = t561 * t12344;
    (t12303, t12305, t12306, t12321, t12322, t12338, t12345)
}
