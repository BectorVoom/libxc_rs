//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2142/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2142(t22881: f64, t6347: f64, t6637: f64, t6888: f64, t19631: f64, t6968: f64, t22705: f64, t28130: f64, t81228: f64, t19748: f64, t1992: f64, t22897: f64) -> (f64, f64, f64, f64) {
    let t97036 = t6888 * t6637 * t22881 * t6347;
    let t97040 = t6888 * t6637 * t6968 * t19631;
    let t97043 = t81228 * t22705 * t28130;
    let t97046 = t1992 * t22897 * t19748;
    (t97036, t97040, t97043, t97046)
}
