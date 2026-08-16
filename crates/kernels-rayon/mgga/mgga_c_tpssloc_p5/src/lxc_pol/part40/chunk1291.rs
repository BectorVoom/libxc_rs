//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1291/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1291(t30263: f64, t576: f64, t2193: f64, t6470: f64, t1851: f64, t8256: f64, t2186: f64, t6483: f64, t29895: f64, t30411: f64, t1453: f64, t2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t110910 = 2.0_f64 * t576 * t30263;
    let t111316 = t6470 * t2193;
    let t111317 = t1851 * t8256;
    let t111322 = t2186 * t6483;
    let t111326 = t29895 * t30411;
    let t111331 = t1453 * t2;
    (t110910, t111316, t111317, t111322, t111326, t111331)
}
