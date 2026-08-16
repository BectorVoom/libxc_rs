//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 392/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk392(t142: f64, t814: f64, t298: f64, t831: f64, t28: f64, t813: f64, t14: f64, t829: f64, t830: f64, t181: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2850 = t142 * t814;
    let t2853 = 0.35616666666666666667e-1_f64 * t298 * t2850 * t831;
    let t2854 = t813 * t28;
    let t2855 = 1.0_f64 / t2854;
    let t2856 = t14 * t2855;
    let t2857 = t829 * t829;
    let t2858 = t2857 * t830;
    let t2860 = 2.0_f64 * t2856 * t2858;
    let t2861 = 1.0_f64 / t181;
    (t2850, t2853, t2855, t2856, t2857, t2858, t2860, t2861)
}
