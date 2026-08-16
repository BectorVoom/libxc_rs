//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1029/1415 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1029(t11247: f64, t14702: f64, t18203: f64, t18219: f64, t18229: f64, t21760: f64, t21764: f64, t21767: f64, t21771: f64, t21774: f64, t21778: f64, t1107: f64) -> (f64, f64) {
    let t21780 = -t11247 + 4.0_f64 / 9.0_f64 * t14702 + 2.0_f64 / 9.0_f64 * t18203 - 2.0_f64 / 3.0_f64 * t18219 - t18229 / 3.0_f64 + 10.0_f64 / 27.0_f64 * t21760 - 4.0_f64 / 3.0_f64 * t21764 - 2.0_f64 / 3.0_f64 * t21767 + 2.0_f64 * t21771 + 2.0_f64 * t21774 + t21778 / 3.0_f64;
    let t21781 = t1107 * t21780;
    (t21780, t21781)
}
