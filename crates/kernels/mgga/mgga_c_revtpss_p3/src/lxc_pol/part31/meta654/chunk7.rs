//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2193/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2193<F: Float>(t1955: F, t27883: F, t1444: F, t25924: F, t27865: F, t27869: F, t27909: F, t30031: F, t30106: F, t5728: F, t7295: F, t94608: F, t94616: F, t94705: F, t97792: F, t97795: F, t97798: F, t97800: F, t97804: F, t97808: F, t97810: F, t97815: F, t97933: F) -> (F, F) {
    let t108225 = t1955 * t27883;
    let t108233 = F::cast_from(0.14634331517634470219e-1_f64) * t97792 + F::cast_from(0.13009920719177044025e-2_f64) * t97795 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t25924 * t30031 * t1444 + F::cast_from(0.26341796731742046394e1_f64) * t27909 * t5728 - t97798 - F::cast_from(0.45699670022203476294e-2_f64) * t97800 - F::cast_from(0.17347256376410398924e1_f64) * t97933 * t27865 + F::cast_from(0.8673628188205199462e0_f64) * t108225 * t27869 - t94608 - t97804 + t97808 + F::cast_from(0.23131639038696784278e-2_f64) * t97810 + F::cast_from(0.11565819519348392139e-2_f64) * t94616 + F::cast_from(0.91399340044406952588e-2_f64) * t97815 - F::cast_from(0.17347256376410398924e1_f64) * t94705 * t30106;
    (t108225, t108233)
}
