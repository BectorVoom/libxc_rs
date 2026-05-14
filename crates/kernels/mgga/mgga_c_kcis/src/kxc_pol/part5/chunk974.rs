//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 974/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk974<F: Float>(t9323: F, t1655: F, t2791: F, t4536: F, t4539: F, t5404: F, t5409: F, t4530: F, t5402: F, t4541: F, t5415: F, t5412: F, t5400: F, t6262: F, t4544: F, t4528: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t18375 = 6.0 * t9323;
    let t18385 = t1655 * t2791;
    let t18413 = t4536 / 8.0;
    let t18414 = t4539 / 8.0;
    let t18415 = t5404 / 8.0;
    let t18416 = t5409 / 8.0;
    let t18417 = t4530 / 8.0;
    let t18418 = t5402 / 8.0;
    let t18419 = t4541 / 8.0;
    let t18422 = t5415 / 8.0;
    let t18423 = t5412 / 8.0;
    let t18424 = t5400 / 8.0;
    let t18425 = t6262 / 8.0;
    let t18426 = t4544 / 8.0;
    let t18427 = 2.0 * t4528;
    (t18375, t18385, t18413, t18414, t18415, t18416, t18417, t18418, t18419, t18422, t18423, t18424, t18425, t18426, t18427)
}
