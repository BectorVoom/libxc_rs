//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1337/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1337(t1369: f64, t32717: f64, t1831: f64, t31165: f64, t5314: f64, t8466: f64, t22804: f64, t32711: f64, t22759: f64, t26318: f64, t6936: f64, t1799: f64, t22690: f64, t22792: f64, t6950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120377 = t32717 * t1369;
    let t120379 = t31165 * t1831;
    let t120381 = t8466 * t5314;
    let t120383 = t22804 * t32711;
    let t120388 = t6936 * t22759 * t26318;
    let t120393 = t22792 * t22690 * t6950 * t1799;
    (t120377, t120379, t120381, t120383, t120388, t120393)
}
