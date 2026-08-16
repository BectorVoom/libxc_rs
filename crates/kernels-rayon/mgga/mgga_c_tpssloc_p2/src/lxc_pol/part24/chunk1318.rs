//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1318/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1318(t2690: f64, t6619: f64, t812: f64, t849: f64, t6620: f64, t9612: f64, t23132: f64, t2617: f64, t23133: f64, t2707: f64, t6621: f64, t9997: f64) -> (f64, f64, f64, f64, f64) {
    let t81763 = t812 * t6619 * t2690;
    let t81764 = t81763 * t849;
    let t81766 = t9612 * t6620;
    let t81767 = t81766 * t849;
    let t81769 = t2617 * t23132;
    let t81770 = t81769 * t849;
    let t81772 = t23133 * t2707;
    let t81774 = t6621 * t9997;
    (t81764, t81767, t81770, t81772, t81774)
}
