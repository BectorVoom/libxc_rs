//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2531/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2531(t13823: f64, t2960: f64, t13816: f64, t2970: f64, t973: f64, t13828: f64, t10224: f64, t4522: f64, t13895: f64, t1599: f64, t2402: f64, t13908: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t48297 = t2960 * t13823;
    let t48302 = t973 * t2970 * t13816;
    let t48317 = t2960 * t13828;
    let t48320 = t973 * t10224 * t4522;
    let t48328 = t2960 * t13895;
    let t48336 = t973 * t2402 * t1599;
    let t48338 = t2960 * t13908;
    (t48297, t48302, t48317, t48320, t48328, t48336, t48338)
}
