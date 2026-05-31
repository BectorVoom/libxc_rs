//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1023/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1023<F: Float>(t530: F, t64: F, t555: F, t491: F, t1505: F, t4182: F, t1502: F, t4188: F, t1504: F, t561: F, t1507: F, t456: F) -> (F, F, F, F, F, F) {
    let t12319 = t64 * t530;
    let t12321 = F::cast_from(1.0_f64) / t555 / t12319;
    let t12322 = t491 * t12321;
    let t12335 = t4182 * t1505;
    let t12338 = t1502 * t4188;
    let t12343 = t1504 * t1504;
    let t12344 = F::cast_from(1.0_f64) / t12343;
    let t12345 = t561 * t12344;
    let t12361 = t1507 * t456;
    (t12321, t12322, t12335, t12338, t12345, t12361)
}
