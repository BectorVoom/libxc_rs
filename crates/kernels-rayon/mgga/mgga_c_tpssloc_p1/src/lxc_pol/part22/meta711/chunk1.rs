//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2309/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2309(t40761: f64, t16689: f64, t4101: f64, t16701: f64, t4205: f64, t20741: f64, t706: f64, t708: f64, t20234: f64, t751: f64, t9897: f64, t13133: f64, t5597: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67176 = 0.10254018858216406658e4_f64 * t40761;
    let t67177 = t16689 * t4101;
    let t67178 = 12.0_f64 * t67177;
    let t67179 = t4205 * t16701;
    let t67180 = 12.0_f64 * t67179;
    let t67181 = t706 * t20741;
    let t67183 = 4.0_f64 * t67181 * t708;
    let t67185 = t9897 * t751 * t20234;
    let t67186 = 24.0_f64 * t67185;
    let t67191 = 12.0_f64 * t13133 * t5597;
    (t67176, t67178, t67180, t67183, t67186, t67191)
}
