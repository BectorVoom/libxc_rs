//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 718/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk718(t2354: f64, t9757: f64, t446: f64, t2459: f64, t684: f64, t1882: f64, t2356: f64, t2336: f64, t2362: f64, t89: f64, t2371: f64, t683: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9758 = t2354 * t9757;
    let t9759 = t446 * t9758;
    let t9761 = t684 * t2459;
    let t9762 = t2354 * t9761;
    let t9763 = t446 * t9762;
    let t9765 = t1882 * t2356;
    let t9768 = t89 * t2336 * t2362;
    let t9770 = t683 * t2371;
    (t9758, t9759, t9761, t9762, t9763, t9765, t9768, t9770)
}
