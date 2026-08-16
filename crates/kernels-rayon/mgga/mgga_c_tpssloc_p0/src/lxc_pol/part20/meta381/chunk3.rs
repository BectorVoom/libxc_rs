//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1741/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1741(t1509: f64, t828: f64, t2647: f64, t13222: f64, t2632: f64) -> (f64, f64, f64) {
    let t13223 = t1509 * t828;
    let t13224 = t13223 * t2647;
    let t13225 = t13222 * t13224;
    let t13228 = t1509 * t2632;
    (t13223, t13225, t13228)
}
