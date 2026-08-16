//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 898/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk898(t1222: f64, t7334: f64, t2141: f64, t3540: f64, t3: f64, t7324: f64, t1184: f64, t52: f64, t460: f64, t3548: f64, t7310: f64, t2127: f64, t3545: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24675 = t7334 * t1222;
    let t24681 = t2141 * t3540 / 6912.0_f64;
    let t24682 = t7324 * t3;
    let t24683 = t52 * t1184;
    let t24684 = t24683 * t460;
    let t24685 = t24682 * t24684;
    let t24690 = t7310 * t3548;
    let t24704 = t2127 * t3545 / 432.0_f64;
    (t24675, t24681, t24682, t24683, t24685, t24690, t24704)
}
