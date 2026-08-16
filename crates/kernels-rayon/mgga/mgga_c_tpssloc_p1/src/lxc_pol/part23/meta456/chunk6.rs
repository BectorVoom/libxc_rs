//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1325/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1325(t76132: f64, t76167: f64, t76193: f64, t76227: f64, t76259: f64, t76295: f64, t76333: f64, t76394: f64, t10080: f64, t1499: f64, t16673: f64, t17027: f64, t20857: f64, t20858: f64, t21014: f64, t226: f64, t235: f64, t255: f64, t40932: f64, t4166: f64, t46524: f64, t5585: f64, t5612: f64, t5617: f64, t5653: f64, t59355: f64, t76086: f64, t76090: f64, t76373: f64, t812: f64) -> (f64, f64) {
    let t76397 = t76132 + t76167 + t76193 + t76227 + t76259 + t76295 + t76333 + t76394;
    let t76414 = -36.0_f64 * t10080 * t76090 * t812 - 6.0_f64 * t17027 * t5612 * t812 - 6.0_f64 * t17027 * t5617 * t812 - 24.0_f64 * t20857 * t46524 * t812 + t226 * t235 * t76397 + 24.0_f64 * t40932 * t76086 * t812 + 12.0_f64 * t5585 * t59355 * t812 + 4.0_f64 * t1499 * t21014 - 6.0_f64 * t16673 * t5653 - 24.0_f64 * t20858 * t4166 + t255 * t76373;
    (t76397, t76414)
}
