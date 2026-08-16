//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1606/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1606(t23041: f64, t831: f64, t2686: f64, t6614: f64, t2627: f64, t59: f64, t240: f64, t812: f64, t2635: f64, t2681: f64, t2617: f64, t6613: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t23042 = t23041 * t831;
    let t23043 = 7.0_f64 / 1152.0_f64 * t23042;
    let t23044 = t6614 * t2686;
    let t23046 = t2627 * t59;
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23049 = t23048 * t2635;
    let t23051 = t6614 * t2681;
    let t23053 = t2617 * t6613;
    (t23042, t23043, t23044, t23046, t23047, t23049, t23051, t23053)
}
