//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1831/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1831(t1307: f64, t19577: f64, t1530: f64, t2379: f64, t22960: f64, t57893: f64, t2745: f64, t25373: f64, t25: f64, t40772: f64, t2749: f64, t1408: f64, t2752: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t86685 = t19577 * t1307;
    let t86706 = t1530 * t2379;
    let t86707 = t22960 * t86706;
    let t86710 = t22960 * t57893;
    let t86713 = t1530 * t2745;
    let t86714 = t25373 * t86713;
    let t86716 = t40772 * t25;
    let t86717 = t1530 * t2749;
    let t86718 = t86716 * t86717;
    let t86721 = t2752 * t1408;
    (t86685, t86706, t86707, t86710, t86713, t86714, t86717, t86718, t86721)
}
