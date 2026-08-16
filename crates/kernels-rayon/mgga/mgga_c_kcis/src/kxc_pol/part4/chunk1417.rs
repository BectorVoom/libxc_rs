//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1417/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1417(t4536: f64, t4539: f64, t5404: f64, t5409: f64, t4530: f64, t5402: f64, t4541: f64, t13034: f64, t13043: f64, t13044: f64, t18373: f64, t18410: f64, t2652: f64, t2798: f64, t2808: f64, t6292: f64, t8: f64) -> f64 {
    let t18413 = t4536 / 8.0_f64;
    let t18414 = t4539 / 8.0_f64;
    let t18415 = t5404 / 8.0_f64;
    let t18416 = t5409 / 8.0_f64;
    let t18417 = t4530 / 8.0_f64;
    let t18418 = t5402 / 8.0_f64;
    let t18419 = t4541 / 8.0_f64;
    let t18420 = t8 * (t18373 + t18410) - t18413 - t18414 + t18415 + t2798 - t13034 + t2652 - t18416 + t18417 + t18418 - t13044 + t13043 + t18419 - t2808 + t6292;
    t18420
}
