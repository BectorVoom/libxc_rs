//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 527/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk527(t2173: f64, t2175: f64, t2187: f64, t352: f64, t828: f64, t832: f64) -> (f64, f64, f64) {
    let t2189 = t2173 - 0.35616666666666666666e-1_f64 * t2175 + 0.53425e-1_f64 * t2187;
    let t2191 = 0.621814e-1_f64 * t2189 * t352;
    let t2192 = t828 * t832;
    (t2189, t2191, t2192)
}
