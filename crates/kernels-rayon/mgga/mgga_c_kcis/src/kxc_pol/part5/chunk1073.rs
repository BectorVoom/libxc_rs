//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1073/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1073(t740: f64, t9323: f64, t1655: f64, t2791: f64, t4536: f64, t4539: f64, t5404: f64, t5409: f64, t4530: f64, t5402: f64, t4541: f64, t5415: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t18374 = 2.0_f64 * t740;
    let t18375 = 6.0_f64 * t9323;
    let t18385 = t1655 * t2791;
    let t18413 = t4536 / 8.0_f64;
    let t18414 = t4539 / 8.0_f64;
    let t18415 = t5404 / 8.0_f64;
    let t18416 = t5409 / 8.0_f64;
    let t18417 = t4530 / 8.0_f64;
    let t18418 = t5402 / 8.0_f64;
    let t18419 = t4541 / 8.0_f64;
    let t18422 = t5415 / 8.0_f64;
    (t18374, t18375, t18385, t18413, t18414, t18415, t18416, t18417, t18418, t18419, t18422)
}
