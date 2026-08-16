//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1418/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1418(t5415: f64, t5412: f64, t5400: f64, t6262: f64, t4544: f64, t4528: f64, t13031: f64, t2653: f64, t2796: f64, t2800: f64, t2805: f64, t3706: f64, t3711: f64, t3714: f64, t4507: f64, t8521: f64) -> f64 {
    let t18422 = t5415 / 8.0_f64;
    let t18423 = t5412 / 8.0_f64;
    let t18424 = t5400 / 8.0_f64;
    let t18425 = t6262 / 8.0_f64;
    let t18426 = t4544 / 8.0_f64;
    let t18427 = 2.0_f64 * t4528;
    let t18428 = 4.0_f64 * t2653 + t13031 - t18422 + t2800 - t18423 - t2796 - t18424 + t8521 - t4507 - t3714 - t3711 - t3706 - t18425 - t18426 + t18427 - t2805;
    t18428
}
