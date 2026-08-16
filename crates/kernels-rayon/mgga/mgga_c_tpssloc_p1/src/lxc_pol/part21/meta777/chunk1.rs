//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2688/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2688(t212: f64, t6330: f64, t2586: f64, t40353: f64, t6347: f64, t12225: f64, t40343: f64, t40347: f64, t40350: f64, t40351: f64, t40356: f64, t40360: f64, t54631: f64, t54633: f64, t54635: f64, t54637: f64, t54639: f64, t54643: f64) -> (f64, f64, f64) {
    let t56463 = t212 * t6330;
    let t56465 = t2586 * t40353 * t56463;
    let t56467 = t212 * t6347;
    let t56469 = t2586 * t12225 * t56467;
    let t56475 = -t40343 + t40347 + t40350 - 0.5185185185185185185e-1_f64 * t54631 + 0.65740740740740740737e-1_f64 * t54633 + 0.77777777777777777775e-2_f64 * t54635 - 0.2111111111111111111e-1_f64 * t54637 + 0.11234567901234567901e0_f64 * t54639 - 0.49999999999999999998e-2_f64 * t56465 + 0.16666666666666666666e-2_f64 * t56469 - 0.19999999999999999999e-1_f64 * t54643 - 0.39999999999999999998e-1_f64 * t40351 - 0.49999999999999999998e-2_f64 * t40356 + 0.16666666666666666666e-2_f64 * t40360;
    (t56463, t56467, t56475)
}
