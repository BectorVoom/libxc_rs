//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 756/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk756(t10235: f64, t829: f64, t2648: f64, t2744: f64, t1882: f64, t2667: f64, t2336: f64, t2671: f64, t89: f64, t2680: f64, t683: f64, t2682: f64, t684: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10236 = t10235 * t829;
    let t10238 = t2648 * t2744;
    let t10243 = t1882 * t2667;
    let t10246 = t89 * t2336 * t2671;
    let t10248 = t683 * t2680;
    let t10249 = t684 * t2682;
    (t10236, t10238, t10243, t10246, t10248, t10249)
}
