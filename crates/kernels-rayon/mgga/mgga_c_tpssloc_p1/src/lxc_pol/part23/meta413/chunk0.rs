//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1231/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1231(t21036: f64, t225: f64, t20852: f64, t252: f64, t1519: f64, t5611: f64, t21013: f64, t814: f64, t20937: f64, t68: f64, t20217: f64, t707: f64, t751: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67344 = t21036 * t225;
    let t67392 = t252 * t20852;
    let t67405 = t1519 * t5611;
    let t67429 = t814 * t21013;
    let t67441 = t20937 * t68;
    let t67463 = t707 * t751 * t20217;
    (t67344, t67392, t67405, t67429, t67441, t67463)
}
