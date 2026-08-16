//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1229/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1229(t16689: f64, t4101: f64, t16701: f64, t4205: f64, t20741: f64, t706: f64, t20234: f64, t751: f64, t9897: f64, t20742: f64, t67: f64, t758: f64) -> (f64, f64, f64, f64, f64) {
    let t67177 = t16689 * t4101;
    let t67179 = t4205 * t16701;
    let t67181 = t706 * t20741;
    let t67185 = t9897 * t751 * t20234;
    let t67209 = t20742 * t67 * t758;
    (t67177, t67179, t67181, t67185, t67209)
}
