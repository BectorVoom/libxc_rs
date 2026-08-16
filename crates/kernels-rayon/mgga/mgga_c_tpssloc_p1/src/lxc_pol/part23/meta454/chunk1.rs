//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1312/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1312(t185: f64, t707: f64, t75912: f64, t58984: f64, t46433: f64, t46439: f64, t1409: f64, t4194: f64, t67469: f64, t59013: f64, t12939: f64, t16716: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t76024 = 4.0_f64 * t707 * t185 * t75912;
    let t76025 = 0.14649157844805236043e-2_f64 * t58984;
    let t76026 = 0.22787578869697033845e-2_f64 * t46433;
    let t76027 = 4.0_f64 * t46439;
    let t76030 = 48.0_f64 * t4194 * t67469 * t1409;
    let t76031 = 72.0_f64 * t59013;
    let t76034 = 144.0_f64 * t12939 * t16716 * t5398;
    (t76024, t76025, t76026, t76027, t76030, t76031, t76034)
}
