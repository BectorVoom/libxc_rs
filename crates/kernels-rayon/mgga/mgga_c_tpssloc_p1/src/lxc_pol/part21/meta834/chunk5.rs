//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2958/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2958(t18059: f64, t225: f64, t1020: f64, t17960: f64, t248: f64, t3101: f64, t13950: f64, t4644: f64, t10508: f64, t3130: f64, t5873: f64, t17611: f64, t3114: f64) -> (f64, f64, f64, f64, f64) {
    let t61646 = t18059 * t225;
    let t61655 = t1020 * t248 * t3101 * t17960;
    let t61659 = t4644 * t13950;
    let t61663 = t3130 * t248 * t10508 * t5873;
    let t61665 = t3114 * t17611;
    (t61646, t61655, t61659, t61663, t61665)
}
