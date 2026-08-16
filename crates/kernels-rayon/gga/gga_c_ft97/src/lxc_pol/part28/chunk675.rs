//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 675/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk675(t376: f64, t6588: f64, t1984: f64, t6615: f64, t5779: f64, t28: f64, t379: f64, t6587: f64, t24080: f64, t1969: f64, t24102: f64, t925: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26788 = t376 * t6588;
    let t26791 = t1984 * t6615;
    let t26792 = t26791 * t5779;
    let t26793 = t28 * t26792;
    let t26800 = t6587 * t379;
    let t26801 = t24080 * t26800;
    let t26805 = t1969 * t24102 * t925;
    (t26788, t26791, t26793, t26800, t26801, t26805)
}
