//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1161/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1161(t32629: f64, t580: f64, t1404: f64, t8919: f64, t131: f64, t32582: f64, t2240: f64, t9239: f64, t9231: f64, t32578: f64, t39063: f64, t39054: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t117693 = t32629 * t580;
    let t117695 = t8919 * t1404;
    let t117709 = t32582 * t131;
    let t117710 = t2240 * t117709;
    let t117727 = t9239 * t117709;
    let t117734 = t9231 * t32582;
    let t117737 = t39063 * t32578;
    let t117757 = t39054 * t32578;
    (t117693, t117695, t117710, t117727, t117734, t117737, t117757)
}
