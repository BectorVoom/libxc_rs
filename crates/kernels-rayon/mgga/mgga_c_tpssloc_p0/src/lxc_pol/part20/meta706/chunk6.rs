//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2696/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2696(t3787: f64, t5318: f64, t1834: f64, t3850: f64, t12248: f64, t12172: f64, t12251: f64, t12267: f64, t1336: f64, t1351: f64, t1352: f64, t16033: f64, t16036: f64, t16047: f64, t16060: f64, t16125: f64, t16127: f64, t3777: f64, t3856: f64, t3898: f64, t40335: f64, t5234: f64, t5250: f64, t5334: f64, t5335: f64, t5339: f64, t5341: f64, t5344: f64, t54854: f64, t54883: f64) -> (f64, f64, f64) {
    let t54905 = t3787 * t5318;
    let t54918 = t1834 * t3850;
    let t54930 = t12248 * t1834;
    let t54959 = -18.0_f64 * t1351 * t16047 * t40335 * t5335 - 6.0_f64 * t12251 * t1336 * t54930 - 3.0_f64 * t1352 * t5344 * t54854 - 3.0_f64 * t16036 * t3856 * t5344 + 6.0_f64 * t5250 * t5334 * t54883 + 6.0_f64 * t12172 * t5234 - 3.0_f64 * t12267 * t5339 - 3.0_f64 * t12267 * t5341 - 3.0_f64 * t16033 * t16125 + 6.0_f64 * t16060 * t3898 - 3.0_f64 * t16127 * t3777;
    (t54905, t54918, t54959)
}
