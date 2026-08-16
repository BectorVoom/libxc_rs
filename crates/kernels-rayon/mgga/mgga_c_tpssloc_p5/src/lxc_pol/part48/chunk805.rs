//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 805/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk805(t2144: f64, t3493: f64, t1246: f64, t3620: f64, t7376: f64, t7375: f64, t23598: f64, t50: f64, t131: f64, t467: f64, t3030: f64, t461: f64) -> (f64, f64, f64, f64) {
    let t24803 = t2144 * t3493;
    let t24804 = t24803 * t1246;
    let t24806 = t3620 * t7376;
    let t24807 = t7375 * t24806;
    let t24810 = t50 * t23598;
    let t24811 = t24810 * t131;
    let t24812 = t24811 * t467;
    let t24813 = t461 * t3030;
    (t24804, t24807, t24812, t24813)
}
