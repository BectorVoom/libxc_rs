//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2193/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2193(t1955: f64, t27883: f64, t1444: f64, t25924: f64, t27865: f64, t27869: f64, t27909: f64, t30031: f64, t30106: f64, t5728: f64, t7295: f64, t94608: f64, t94616: f64, t94705: f64, t97792: f64, t97795: f64, t97798: f64, t97800: f64, t97804: f64, t97808: f64, t97810: f64, t97815: f64, t97933: f64) -> (f64, f64) {
    let t108225 = t1955 * t27883;
    let t108233 = 0.14634331517634470219e-1_f64 * t97792 + 0.13009920719177044025e-2_f64 * t97795 - 0.26020884564615598386e1_f64 * t7295 * t25924 * t30031 * t1444 + 0.26341796731742046394e1_f64 * t27909 * t5728 - t97798 - 0.45699670022203476294e-2_f64 * t97800 - 0.17347256376410398924e1_f64 * t97933 * t27865 + 0.8673628188205199462e0_f64 * t108225 * t27869 - t94608 - t97804 + t97808 + 0.23131639038696784278e-2_f64 * t97810 + 0.11565819519348392139e-2_f64 * t94616 + 0.91399340044406952588e-2_f64 * t97815 - 0.17347256376410398924e1_f64 * t94705 * t30106;
    (t108225, t108233)
}
