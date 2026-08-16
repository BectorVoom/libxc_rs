//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2201/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2201(t13261: f64, t4166: f64, t118: f64, t2375: f64, t5522: f64, t16575: f64, t706: f64, t16710: f64, t2663: f64, t157: f64, t46387: f64, t12939: f64, t5392: f64, t607: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t58904 = t4166 * t13261;
    let t58972 = t5522 * t118 * t2375;
    let t58976 = t706 * t16575;
    let t58984 = t16710 * t2663;
    let t58994 = t46387 * t157;
    let t59004 = t12939 * t750 * t5392 * t607;
    (t58904, t58972, t58976, t58984, t58994, t59004)
}
