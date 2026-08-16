//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 806/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk806(t2631: f64, t828: f64, t232: f64, t819: f64, t820: f64, t2628: f64, t835: f64, t812: f64, t2635: f64, t2690: f64, t815: f64, t831: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9660 = t2631 * t828;
    let t9661 = t9660 * t232;
    let t9663 = t819 * t820 * t9661;
    let t9666 = t2628 * t835;
    let t9667 = t812 * t9666;
    let t9668 = t9667 * t2635;
    let t9670 = t815 * t2690;
    let t9671 = t812 * t9670;
    let t9672 = t9671 * t831;
    (t9660, t9661, t9663, t9666, t9667, t9668, t9670, t9671, t9672)
}
