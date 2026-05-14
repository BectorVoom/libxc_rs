//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1236/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1236<F: Float>(t4536: F, t4539: F, t5404: F, t5409: F, t4530: F, t5402: F, t4541: F, t13034: F, t13043: F, t13044: F, t18373: F, t18410: F, t2652: F, t2798: F, t2808: F, t6292: F, t8: F) -> (F,) {
    let t18413 = t4536 / 8.0;
    let t18414 = t4539 / 8.0;
    let t18415 = t5404 / 8.0;
    let t18416 = t5409 / 8.0;
    let t18417 = t4530 / 8.0;
    let t18418 = t5402 / 8.0;
    let t18419 = t4541 / 8.0;
    let t18420 = t8 * (t18373 + t18410) - t18413 - t18414 + t18415 + t2798 - t13034 + t2652 - t18416 + t18417 + t18418 - t13044 + t13043 + t18419 - t2808 + t6292;
    (t18420,)
}
