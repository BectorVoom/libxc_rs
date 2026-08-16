//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1968/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1968(t21723: f64, t3315: f64, t11190: f64, t11444: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64) -> (f64, f64, f64) {
    let t21961 = t21723 * t3315;
    let t21963 = 0.96491876992155210402e2_f64 * t11190 * t21961;
    let t21975 = -t11444 + 0.2283111111111111111e-1_f64 * t14702 + 0.11415555555555555555e-1_f64 * t18203 - 0.34246666666666666665e-1_f64 * t18219 - 0.17123333333333333333e-1_f64 * t18229 + 0.19025925925925925925e-1_f64 * t21760 - 0.68493333333333333331e-1_f64 * t21764 - 0.34246666666666666665e-1_f64 * t21767 + 0.10274e0_f64 * t21771 + 0.10274e0_f64 * t21774 + 0.17123333333333333333e-1_f64 * t21778;
    (t21961, t21963, t21975)
}
