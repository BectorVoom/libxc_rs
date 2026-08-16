//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1124/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1124(t1369: f64, t22788: f64, t6597: f64, t6924: f64, t281: f64, t1307: f64, t1361: f64, t22690: f64, t547: f64, t6546: f64, t1329: f64, t3770: f64, t6916: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22789 = t22788 * t1369;
    let t22791 = t6597 * t6924;
    let t22792 = t22791 * t281;
    let t22794 = t22690 * t1361 * t1307;
    let t22795 = t22792 * t22794;
    let t22797 = t6546 * t547;
    let t22798 = t22797 * t1329;
    let t22799 = 7.0_f64 / 72.0_f64 * t22798;
    let t22800 = t6916 * t3770;
    (t22789, t22791, t22792, t22794, t22795, t22797, t22799, t22800)
}
