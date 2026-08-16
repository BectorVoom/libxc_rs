//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2126/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2126(t27989: f64, t94802: f64, t25899: f64, t98303: f64, t1444: f64, t1904: f64, t25924: f64, t26079: f64, t26081: f64, t27837: f64, t27909: f64, t28002: f64, t4003: f64, t4132: f64, t7295: f64, t7298: f64, t94906: f64, t94909: f64, t94911: f64, t94914: f64, t94917: f64, t94919: f64, t94922: f64, t94931: f64, t97909: f64, t98050: f64) -> f64 {
    let t98390 = 0.25702851531048074406e-1_f64 * t94802 * t27989;
    let t98399 = 0.25702851531048074406e-1_f64 * t25899 * t98303;
    let t98414 = t98390 + 0.25702851531048074406e-1_f64 * t94909 + 0.12851425765524037203e-1_f64 * t94911 + 0.34270468708064099208e-2_f64 * t94914 - 0.65854491829355115987e0_f64 * t94906 * t1904 + t94917 - 0.48186823267806663678e-3_f64 * t94919 - 0.14456046980341999104e-1_f64 * t94922 + t98399 - 0.65854491829355115987e0_f64 * t27909 * t4132 - t94931 + 0.17347256376410398924e1_f64 * t98050 * t7298 - 0.52041769129231196772e1_f64 * t7295 * t25924 * t28002 * t1444 - 0.8673628188205199462e0_f64 * t7295 * t26079 * t97909 * t4003 - 0.8673628188205199462e0_f64 * t27837 * t26081;
    t98414
}
