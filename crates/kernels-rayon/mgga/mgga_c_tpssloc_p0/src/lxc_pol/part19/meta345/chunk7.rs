//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1241/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1241(t10046: f64, t814: f64, t10016: f64, t10058: f64, t10073: f64, t10081: f64, t10094: f64, t13453: f64, t255: f64, t2613: f64, t2617: f64, t2728: f64, t2732: f64, t2740: f64, t41231: f64, t41333: f64, t41368: f64, t41429: f64, t808: f64, t812: f64, t860: f64, t863: f64, t9661: f64) -> (f64, f64) {
    let t41520 = t814 * t10046;
    let t41549 = 6.0_f64 * t2728 * t41368 * t812 - 4.0_f64 * t2732 * t812 * t9661 - t41333 * t812 * t860 - 3.0_f64 * t41429 * t812 * t860 + 4.0_f64 * t10016 * t863 + 4.0_f64 * t10058 * t808 - 12.0_f64 * t10073 * t2617 - 24.0_f64 * t10081 * t2617 + 24.0_f64 * t10094 * t13453 + t255 * t41231 + 6.0_f64 * t2613 * t2740;
    (t41520, t41549)
}
