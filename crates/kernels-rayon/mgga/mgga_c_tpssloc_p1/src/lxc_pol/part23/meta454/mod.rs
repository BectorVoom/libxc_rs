//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta454 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1312;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta454(t16693: f64, t20749: f64, t46376: f64, t16689: f64, t5597: f64, t39585: f64, t39590: f64, t39593: f64, t41254: f64, t75943: f64, t75950: f64, t75951: f64, t75952: f64, t185: f64, t707: f64, t75912: f64, t58984: f64, t46433: f64, t46439: f64, t1409: f64, t4194: f64, t67469: f64, t59013: f64, t12939: f64, t16716: f64, t5398: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76017, t76018, t76020, t76021) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1311(t16693, t20749, t46376, t16689, t5597, t39585, t39590, t39593, t41254, t75943, t75950, t75951, t75952);
        let (t76024, t76025, t76026, t76027, t76030, t76031, t76034) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1312(t185, t707, t75912, t58984, t46433, t46439, t1409, t4194, t67469, t59013, t12939, t16716, t5398);
    (t76017, t76018, t76020, t76021, t76024, t76025, t76026, t76027, t76030, t76031, t76034)
}
