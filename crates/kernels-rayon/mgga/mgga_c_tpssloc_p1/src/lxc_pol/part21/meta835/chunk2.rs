//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2964/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2964(t17919: f64, t3070: f64, t42488: f64, t1022: f64, t3966: f64, t360: f64, t1041: f64, t10868: f64, t248: f64, t5685: f64, t14134: f64, t4644: f64) -> (f64, f64, f64, f64) {
    let t61768 = t3070 * t42488 * t17919;
    let t61774 = t3966 * t1022;
    let t61775 = t61774 * t360;
    let t61782 = t1041 * t248 * t10868 * t5685;
    let t61784 = t4644 * t14134;
    (t61768, t61775, t61782, t61784)
}
