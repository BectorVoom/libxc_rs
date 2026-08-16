//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2696/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2696<F: Float>(t3787: F, t5318: F, t1834: F, t3850: F, t12248: F, t12172: F, t12251: F, t12267: F, t1336: F, t1351: F, t1352: F, t16033: F, t16036: F, t16047: F, t16060: F, t16125: F, t16127: F, t3777: F, t3856: F, t3898: F, t40335: F, t5234: F, t5250: F, t5334: F, t5335: F, t5339: F, t5341: F, t5344: F, t54854: F, t54883: F) -> (F, F, F) {
    let t54905 = t3787 * t5318;
    let t54918 = t1834 * t3850;
    let t54930 = t12248 * t1834;
    let t54959 = -F::cast_from(18.0_f64) * t1351 * t16047 * t40335 * t5335 - F::cast_from(6.0_f64) * t12251 * t1336 * t54930 - F::cast_from(3.0_f64) * t1352 * t5344 * t54854 - F::cast_from(3.0_f64) * t16036 * t3856 * t5344 + F::cast_from(6.0_f64) * t5250 * t5334 * t54883 + F::cast_from(6.0_f64) * t12172 * t5234 - F::cast_from(3.0_f64) * t12267 * t5339 - F::cast_from(3.0_f64) * t12267 * t5341 - F::cast_from(3.0_f64) * t16033 * t16125 + F::cast_from(6.0_f64) * t16060 * t3898 - F::cast_from(3.0_f64) * t16127 * t3777;
    (t54905, t54918, t54959)
}
