//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2236/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2236(t17884: f64, t3117: f64, t18029: f64, t3108: f64, t17919: f64, t3070: f64, t42488: f64, t1041: f64, t10868: f64, t248: f64, t5685: f64, t14134: f64, t4644: f64) -> (f64, f64, f64, f64, f64) {
    let t61744 = t3117 * t17884;
    let t61754 = t18029 * t3108;
    let t61768 = t3070 * t42488 * t17919;
    let t61782 = t1041 * t248 * t10868 * t5685;
    let t61784 = t4644 * t14134;
    (t61744, t61754, t61768, t61782, t61784)
}
