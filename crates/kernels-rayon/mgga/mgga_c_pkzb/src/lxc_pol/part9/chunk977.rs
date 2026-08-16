//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 977/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk977(t2901: f64, t7653: f64, t302: f64, t2900: f64, t6022: f64, t5953: f64, t7663: f64) -> (f64, f64, f64, f64, f64) {
    let t7728 = t7653 * t2901;
    let t7729 = t302 * t7728;
    let t7732 = t2900 * t6022;
    let t7733 = t302 * t7732;
    let t7736 = t5953 * t7663;
    (t7728, t7729, t7732, t7733, t7736)
}
