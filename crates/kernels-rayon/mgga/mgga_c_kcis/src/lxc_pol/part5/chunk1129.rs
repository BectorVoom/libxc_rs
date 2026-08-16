//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1129/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1129(t13710: f64, t13712: f64, t13717: f64, t13842: f64, t18645: f64, t18650: f64, t18655: f64, t18659: f64, t18661: f64, t18664: f64, t18667: f64, t18669: f64, t18674: f64, t18679: f64, t18683: f64, t9691: f64, t9790: f64) -> f64 {
    let t19040 = -t9790 - 0.79148148148148148147e-2_f64 * t9691 - 0.15829629629629629629e-1_f64 * t13710 + 0.79148148148148148147e-2_f64 * t13712 - t13842 + 0.23744444444444444444e-1_f64 * t13717 + 0.39574074074074074073e-2_f64 * t18645 - 0.19787037037037037037e-1_f64 * t18650 + 0.71233333333333333332e-1_f64 * t18655 - 0.47488888888888888888e-1_f64 * t18659 - 0.11872222222222222222e-1_f64 * t18661 - 0.10685e0_f64 * t18664 + 0.14246666666666666666e0_f64 * t18667 + 0.5936111111111111111e-2_f64 * t18669 - 0.11872222222222222222e-1_f64 * t18674 + 0.35616666666666666666e-1_f64 * t18679 - 0.17808333333333333333e-1_f64 * t18683;
    t19040
}
