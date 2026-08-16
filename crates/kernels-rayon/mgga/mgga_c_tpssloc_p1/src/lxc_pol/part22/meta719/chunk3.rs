//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2330/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2330(t4233: f64, t9975: f64, t13397: f64, t1510: f64, t16679: f64, t16815: f64, t16816: f64, t16828: f64, t16830: f64, t16935: f64, t17027: f64, t17028: f64, t20806: f64, t2617: f64, t4166: f64, t4234: f64, t4281: f64, t59347: f64, t67358: f64, t67441: f64, t67568: f64, t812: f64, t860: f64, t861: f64) -> (f64, f64) {
    let t67578 = t9975 * t4233;
    let t67582 = -18.0_f64 * t13397 * t16815 * t67578 - 18.0_f64 * t13397 * t16816 * t67358 - 3.0_f64 * t1510 * t59347 * t812 + 18.0_f64 * t16815 * t16935 * t4281 - 3.0_f64 * t17027 * t4234 * t812 - t67568 * t812 * t860 - 6.0_f64 * t16679 * t4166 - 3.0_f64 * t16828 * t16830 - 3.0_f64 * t17028 * t4166 - 3.0_f64 * t20806 * t2617 - t67441 * t861;
    (t67578, t67582)
}
