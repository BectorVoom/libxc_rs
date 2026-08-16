//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 388/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk388(t1751: f64, t466: f64, t1734: f64, t491: f64, t1246: f64, t493: f64, t1244: f64, t1729: f64, t470: f64, t494: f64, t1241: f64, t1238: f64, t1721: f64, t498: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t1752 = t466 * t1751;
    let t1755 = t491 * t1734;
    let t1756 = t1755 * t1246;
    let t1758 = t493 * t1751;
    let t1760 = t1244 * t1756 + t1729 * t494 + t1758 * t470;
    let t1761 = t1241 * t1760;
    let t1763 = -t1238 * t1761 + t1721 * t498 + t1752 * t498;
    (t1752, t1755, t1756, t1758, t1760, t1761, t1763)
}
