//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1152/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk1152(t331: f64, t6310: f64, t6272: f64, t829: f64, t1646: f64, t167: f64, t6452: f64, t738: f64, t6455: f64, t743: f64, t6458: f64, t733: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19381 = t331 * t6310;
    let t19396 = t6272 * t829;
    let t19399 = t1646 * t167;
    let t19416 = t738 * t6452;
    let t19418 = t743 * t6455;
    let t19420 = t733 * t6458;
    (t19381, t19396, t19399, t19416, t19418, t19420)
}
