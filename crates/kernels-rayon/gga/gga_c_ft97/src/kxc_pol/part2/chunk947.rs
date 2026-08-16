//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 947/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk947(t2876: f64, t3690: f64, t10409: f64, t446: f64, t3699: f64, t2665: f64, t2680: f64, t4129: f64, t824: f64, t193: f64, t89: f64, t2739: f64, t4056: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14686 = t3690 * t2876;
    let t14687 = t10409 * t14686;
    let t14688 = t446 * t14687;
    let t14690 = t3699 * t2876;
    let t14691 = t2665 * t14690;
    let t14692 = t446 * t14691;
    let t14694 = t2680 * t4129;
    let t14695 = t14694 * t824;
    let t14697 = t89 * t193 * t14695;
    let t14699 = t4056 * t2739;
    (t14686, t14688, t14690, t14692, t14697, t14699)
}
