//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta764 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2640;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2641;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta764(t2559: f64, t5194: f64, t5198: f64, t118: f64, t16018: f64, t3739: f64, t794: f64, t16081: f64, t16086: f64, t12214: f64, t67: f64, t792: f64, t16095: f64, t3734: f64, t686: f64, t133: f64, t1799: f64, t40369: f64, t6600: f64, t131: f64, t205: f64, t40024: f64, t1336: f64, t242: f64, t40042: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t54701, t54705, t54711, t54718) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2640(t2559, t5194, t5198, t118, t16018, t3739, t794, t16081, t16086, t12214, t67, t792);
        let (t54721, t54725, t54728, t54744) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2641(t16095, t3734, t54718, t686, t133, t1799, t40369, t6600, t131, t205, t40024, t1336, t242, t40042);
    (t54701, t54705, t54711, t54718, t54721, t54725, t54728, t54744)
}
