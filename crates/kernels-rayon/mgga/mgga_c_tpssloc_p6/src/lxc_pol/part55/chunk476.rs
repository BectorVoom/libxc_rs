//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 476/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk476(t59: f64, t591: f64, t207: f64, t795: f64, t154: f64, t244: f64, t205: f64, t786: f64, t792: f64, t118: f64, t776: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t2566 = t59 * t591;
    let t2569 = 0.26388888888888888888e-2_f64 * t2566 * t207 * t795;
    let t2570 = t154 * t244;
    let t2571 = t205 * t2570;
    let t2576 = t792 * t786;
    let t2578 = t118 * t794 * t776;
    (t2566, t2569, t2570, t2571, t2576, t2578)
}
