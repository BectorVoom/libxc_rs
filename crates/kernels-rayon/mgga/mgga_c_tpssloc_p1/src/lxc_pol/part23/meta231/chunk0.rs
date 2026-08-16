//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 881/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk881(t5519: f64, t706: f64, t13115: f64, t157: f64, t5398: f64, t751: f64, t707: f64, t5522: f64, t67: f64, t758: f64, t184: f64, t5392: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t16689 = t706 * t5519;
    let t16693 = t13115 * t157;
    let t16701 = t751 * t5398;
    let t16702 = t707 * t16701;
    let t16710 = t5522 * t67;
    let t16711 = t16710 * t758;
    let t16716 = t184 * t5392;
    (t16689, t16693, t16701, t16702, t16710, t16711, t16716)
}
