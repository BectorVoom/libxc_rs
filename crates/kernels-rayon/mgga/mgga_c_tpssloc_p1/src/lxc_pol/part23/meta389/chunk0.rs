//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1193/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1193(t15971: f64, t588: f64, t12364: f64, t5234: f64, t1811: f64, t40005: f64, t40406: f64, t5202: f64, t1804: f64, t16118: f64, t9577: f64, t133: f64, t1799: f64, t40369: f64, t6600: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t54477 = t588 * t15971;
    let t54532 = t5234 * t12364;
    let t54582 = t40005 * t1811;
    let t54633 = t40406 * t5202;
    let t54639 = t40005 * t1804;
    let t54663 = t9577 * t16118;
    let t54725 = t40369 * t133 * t6600 * t1799;
    (t54477, t54532, t54582, t54633, t54639, t54663, t54725)
}
