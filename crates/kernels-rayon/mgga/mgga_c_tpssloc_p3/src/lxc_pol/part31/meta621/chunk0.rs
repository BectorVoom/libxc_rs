//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1874/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1874(t19631: f64, t6637: f64, t6888: f64, t6968: f64, t22705: f64, t28130: f64, t81228: f64, t19748: f64, t1992: f64, t22897: f64, t22704: f64, t28134: f64, t80798: f64) -> (f64, f64, f64, f64) {
    let t97040 = t6888 * t6637 * t6968 * t19631;
    let t97043 = t81228 * t22705 * t28130;
    let t97046 = t1992 * t22897 * t19748;
    let t97049 = t22704 * t80798 * t28134;
    (t97040, t97043, t97046, t97049)
}
