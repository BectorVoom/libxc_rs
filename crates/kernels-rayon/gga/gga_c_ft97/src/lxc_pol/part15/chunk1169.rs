//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1169/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1169(t19759: f64, t5362: f64, t91: f64, t4973: f64, t5225: f64, t10248: f64, t446: f64, t1091: f64, t22161: f64, t2665: f64, t5299: f64, t2857: f64, t88184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t89820 = t91 * t19759 * t5362;
    let t89822 = t4973 * t5225;
    let t89824 = t446 * t10248 * t89822;
    let t89826 = t1091 * t22161;
    let t89828 = t446 * t2665 * t89826;
    let t89832 = t4973 * t5299;
    let t89834 = t446 * t2665 * t89832;
    let t89837 = t446 * t2857 * t88184;
    (t89820, t89822, t89824, t89826, t89828, t89832, t89834, t89837)
}
