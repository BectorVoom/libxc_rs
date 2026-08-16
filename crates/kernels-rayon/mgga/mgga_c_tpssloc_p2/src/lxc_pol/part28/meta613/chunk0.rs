//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1927/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1927(t26421: f64, t26446: f64, t3734: f64, t90591: f64, t22751: f64, t26389: f64, t1992: f64, t22897: f64, t3792: f64, t90870: f64, t26467: f64, t6914: f64) -> (f64, f64, f64, f64) {
    let t91052 = t90591 * t26446 * t26421 * t3734;
    let t91064 = t22751 * t26389;
    let t91074 = t1992 * t22897 * t90870 * t3792;
    let t91076 = t6914 * t26467;
    (t91052, t91064, t91074, t91076)
}
