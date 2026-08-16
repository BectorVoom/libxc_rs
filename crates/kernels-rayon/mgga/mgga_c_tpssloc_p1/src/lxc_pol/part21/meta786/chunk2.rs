//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2728/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2728(t1834: f64, t5286: f64, t12240: f64, t1352: f64, t16036: f64, t16037: f64, t16041: f64, t16047: f64, t16048: f64, t16055: f64, t16419: f64, t19654: f64, t19661: f64, t19735: f64, t19736: f64, t19739: f64, t19743: f64, t19810: f64, t3793: f64, t3851: f64, t40335: f64, t5334: f64, t5344: f64) -> (f64, f64) {
    let t57499 = t1834 * t5286;
    let t57526 = 6.0_f64 * t12240 * t19743 * t5334 - 4.0_f64 * t1352 * t5344 * t57499 + 8.0_f64 * t16036 * t19735 * t5334 - 12.0_f64 * t16047 * t16048 * t19739 - 6.0_f64 * t16047 * t19743 * t40335 + 12.0_f64 * t19739 * t3793 * t5334 - 2.0_f64 * t19739 * t3851 * t5344 - t19743 * t3851 * t5344 + 8.0_f64 * t16037 * t19654 + 8.0_f64 * t16041 * t19654 + 4.0_f64 * t16055 * t19661 + 8.0_f64 * t16055 * t19736 - 4.0_f64 * t16419 * t19810;
    (t57499, t57526)
}
