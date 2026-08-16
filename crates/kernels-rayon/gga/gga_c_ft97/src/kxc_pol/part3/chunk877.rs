//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 877/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk877(t17630: f64, t363: f64, t12122: f64, t358: f64, t8680: f64, t643: f64, t12116: f64, t15734: f64, t15739: f64, t15744: f64, t15748: f64, t15750: f64, t15754: f64, t15758: f64, t15760: f64, t15765: f64, t15770: f64, t15774: f64) -> (f64, f64, f64, f64) {
    let t17631 = t17630 * t363;
    let t17632 = t12122 * t17631;
    let t17636 = t8680 * t358;
    let t17638 = t17636 * t17630 * t643;
    let t17641 = t12116 * t17631;
    let t17666 = 0.3209574074074074074e-1_f64 * t15734 - 0.1604787037037037037e0_f64 * t15739 + 0.57772333333333333332e0_f64 * t15744 - 0.38514888888888888888e0_f64 * t15748 - 0.9628722222222222222e-1_f64 * t15750 - 0.86658499999999999998e0_f64 * t15754 + 0.11554466666666666666e1_f64 * t15758 + 0.4814361111111111111e-1_f64 * t15760 - 0.9628722222222222222e-1_f64 * t15765 + 0.28886166666666666666e0_f64 * t15770 - 0.14443083333333333333e0_f64 * t15774;
    (t17632, t17638, t17641, t17666)
}
