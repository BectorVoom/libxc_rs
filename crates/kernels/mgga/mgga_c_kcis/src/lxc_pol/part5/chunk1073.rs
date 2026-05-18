//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1073/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1073<F: Float>(t740: F, t9323: F, t1655: F, t2791: F, t4536: F, t4539: F, t5404: F, t5409: F, t4530: F, t5402: F, t4541: F, t5415: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t18374 = F::new(2.0) * t740;
    let t18375 = F::new(6.0) * t9323;
    let t18385 = t1655 * t2791;
    let t18413 = t4536 / F::new(8.0);
    let t18414 = t4539 / F::new(8.0);
    let t18415 = t5404 / F::new(8.0);
    let t18416 = t5409 / F::new(8.0);
    let t18417 = t4530 / F::new(8.0);
    let t18418 = t5402 / F::new(8.0);
    let t18419 = t4541 / F::new(8.0);
    let t18422 = t5415 / F::new(8.0);
    (t18374, t18375, t18385, t18413, t18414, t18415, t18416, t18417, t18418, t18419, t18422)
}
