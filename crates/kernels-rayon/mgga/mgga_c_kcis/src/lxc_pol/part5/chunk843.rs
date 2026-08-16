//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 843/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk843(t1282: f64, t187: f64, t1872: f64, t3669: f64, t437: f64, t5360: f64, t6635: f64, t6637: f64, t6640: f64, t6736: f64, t6856: f64, t6860: f64, t6879: f64) -> f64 {
    let t6883 = t6635 - t6637 + t6640 - t6736 + t187 * (-t1282 * t6879 - 2.0_f64 * t1872 * t5360 + 2.0_f64 * t3669 * t6860 + t437 * t6856 - t6635 + t6637 - t6640 + t6736);
    t6883
}
