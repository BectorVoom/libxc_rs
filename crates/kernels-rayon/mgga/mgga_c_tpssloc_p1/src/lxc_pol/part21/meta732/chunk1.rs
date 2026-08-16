//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2590/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2590(t3447: f64, t44579: f64, t4904: f64, t11545: f64, t134: f64, t461: f64, t14726: f64, t11579: f64, t15338: f64, t4899: f64, t4928: f64, t11570: f64, t12648: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t52127 = t3447 * t44579 * t4904;
    let t52133 = t134 * t11545 * t461;
    let t52135 = t3447 * t52133 * t14726;
    let t52138 = t3447 * t15338 * t11579;
    let t52140 = t4899 * t4928;
    let t52161 = t11570 * t12648;
    (t52127, t52133, t52135, t52138, t52140, t52161)
}
