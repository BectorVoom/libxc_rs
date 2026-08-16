//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1107/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1107(t13710: f64, t13713: f64, t13715: f64, t13717: f64, t18645: f64, t18650: f64, t18655: f64, t18659: f64, t18661: f64, t18664: f64, t18667: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t9691: f64, t9736: f64) -> f64 {
    let t18685 = -t9736 - 4.0_f64 / 27.0_f64 * t9691 - 8.0_f64 / 27.0_f64 * t13710 + t13713 - t13715 + 4.0_f64 / 9.0_f64 * t13717 + 2.0_f64 / 27.0_f64 * t18645 - 10.0_f64 / 27.0_f64 * t18650 + 4.0_f64 / 3.0_f64 * t18655 - 8.0_f64 / 9.0_f64 * t18659 - 2.0_f64 / 9.0_f64 * t18661 - 2.0_f64 * t18664 + 8.0_f64 / 3.0_f64 * t18667 + t18669 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t18674 + 2.0_f64 / 3.0_f64 * t18679 - t18683 / 3.0_f64;
    t18685
}
