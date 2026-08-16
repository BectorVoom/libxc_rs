//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2729/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2729(t16046: f64, t1814: f64, t1824: f64, t5318: f64, t1351: f64, t19735: f64, t12240: f64, t16033: f64, t16047: f64, t16048: f64, t16049: f64, t16052: f64, t16055: f64, t16125: f64, t19654: f64, t19660: f64, t19740: f64, t19743: f64, t19763: f64, t19810: f64, t5230: f64, t5250: f64, t5334: f64, t5335: f64, t5343: f64, t5345: f64, t54963: f64, t56666: f64, t57147: f64, t57499: f64) -> (f64, f64) {
    let t57530 = t1814 * t16046;
    let t57545 = t5318 * t1824;
    let t57554 = t19735 * t1351;
    let t57564 = 2.0_f64 * t12240 * t19660 * t5334 - 36.0_f64 * t16047 * t16048 * t19743 + 24.0_f64 * t19743 * t54963 * t56666 - 4.0_f64 * t5230 * t5343 * t5345 + 8.0_f64 * t5250 * t5334 * t57499 + 8.0_f64 * t5250 * t5334 * t57545 + 4.0_f64 * t5334 * t5335 * t57147 + 24.0_f64 * t5334 * t5335 * t57554 - 2.0_f64 * t16033 * t19763 - 12.0_f64 * t16049 * t57530 + 12.0_f64 * t16052 * t19654 + 8.0_f64 * t16055 * t19740 - 2.0_f64 * t16125 * t19810;
    (t57545, t57564)
}
