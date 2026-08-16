//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1235/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1235(t1755: f64, t5079: f64, t1215: f64, t1751: f64, t1246: f64, t493: f64, t5052: f64, t1201: f64, t1244: f64, t1247: f64, t1249: f64, t1729: f64, t1756: f64, t1758: f64, t3604: f64, t3610: f64, t3624: f64, t470: f64, t494: f64, t4964: f64, t5064: f64, t5069: f64, t5073: f64, t5076: f64) -> (f64, f64, f64, f64, f64) {
    let t5080 = t1755 * t5079;
    let t5083 = t1751 * t1215;
    let t5084 = t5083 * t1246;
    let t5086 = t493 * t5052;
    let t5088 = t1201 * t1758 + t1244 * t5073 + t1244 * t5076 + t1244 * t5084 + t1247 * t5064 + t1249 * t1729 + t1756 * t3604 + 2.0_f64 * t3610 * t5069 - t3624 * t5080 + t470 * t5086 + t494 * t4964;
    (t5080, t5083, t5084, t5086, t5088)
}
