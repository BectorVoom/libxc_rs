//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 370/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk370(t1196: f64, t607: f64, t974: f64, t1190: f64, t225: f64, t68: f64) -> (f64, f64, f64, f64) {
    let t1197 = t1196 * t607;
    let t1198 = t974 * t1197;
    let t1201 = t1190 * t225;
    let t1202 = t1201 * t68;
    (t1197, t1198, t1201, t1202)
}
