//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 938/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk938(t181: f64, t686: f64, t781: f64, t756: f64, t118: f64, t753: f64, t2375: f64, t2371: f64, t677: f64, t2374: f64, t2535: f64, t2528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9874 = t686 * t781 * t181;
    let t9876 = 0.56968947174242584612e-3_f64 * t756 * t9874;
    let t9879 = t753 * t118;
    let t9880 = t9879 * t2375;
    let t9882 = t677 * t2371;
    let t9884 = 0.32530743900905219526e-1_f64 * t2374 * t9882;
    let t9885 = t677 * t2535;
    let t9887 = 0.16265371950452609763e-1_f64 * t2374 * t9885;
    let t9888 = t677 * t2528;
    (t9874, t9876, t9880, t9882, t9884, t9885, t9887, t9888)
}
