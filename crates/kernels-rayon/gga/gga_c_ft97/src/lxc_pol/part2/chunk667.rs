//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 667/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk667(t1636: f64, t665: f64, t670: f64, t89: f64, t2404: f64, t675: f64, t1882: f64, t2356: f64, t2336: f64, t2362: f64, t2371: f64, t683: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9733 = t1636 * t665;
    let t9735 = t89 * t9733 * t670;
    let t9744 = t2404 * t675;
    let t9765 = t1882 * t2356;
    let t9768 = t89 * t2336 * t2362;
    let t9770 = t683 * t2371;
    (t9733, t9735, t9744, t9765, t9768, t9770)
}
