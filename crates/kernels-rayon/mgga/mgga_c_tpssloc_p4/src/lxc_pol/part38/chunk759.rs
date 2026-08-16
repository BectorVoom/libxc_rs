//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 759/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk759(t31: f64, t3966: f64, t65: f64, t1410: f64, t628: f64, t1426: f64, t608: f64, t1409: f64, t2267: f64, t607: f64, t43: f64, t2274: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3967 = t31 * t3966;
    let t3968 = t3967 * t65;
    let t3971 = t1410 * t628;
    let t3976 = t608 * t1426;
    let t3981 = t2267 * t1409;
    let t3982 = t3981 * t607;
    let t3985 = t43 * t3966;
    let t3990 = t2274 * t1409;
    (t3967, t3968, t3971, t3976, t3981, t3982, t3985, t3990)
}
