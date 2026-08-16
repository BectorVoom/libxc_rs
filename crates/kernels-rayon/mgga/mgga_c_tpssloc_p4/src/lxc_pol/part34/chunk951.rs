//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 951/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk951(t11459: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t423: f64) -> f64 {
    let t21988 = -t11459 + 0.23744444444444444444e-1_f64 * t14702 + 0.11872222222222222222e-1_f64 * t18203 - 0.35616666666666666666e-1_f64 * t18219 - 0.17808333333333333333e-1_f64 * t18229 + 0.19787037037037037037e-1_f64 * t21760 - 0.71233333333333333332e-1_f64 * t21764 - 0.35616666666666666666e-1_f64 * t21767 + 0.10685e0_f64 * t21771 + 0.10685e0_f64 * t21774 + 0.17808333333333333333e-1_f64 * t21778;
    let t21990 = 0.621814e-1_f64 * t21988 * t423;
    t21990
}
