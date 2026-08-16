//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2688/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2688<F: Float>(t212: F, t6330: F, t2586: F, t40353: F, t6347: F, t12225: F, t40343: F, t40347: F, t40350: F, t40351: F, t40356: F, t40360: F, t54631: F, t54633: F, t54635: F, t54637: F, t54639: F, t54643: F) -> (F, F, F) {
    let t56463 = t212 * t6330;
    let t56465 = t2586 * t40353 * t56463;
    let t56467 = t212 * t6347;
    let t56469 = t2586 * t12225 * t56467;
    let t56475 = -t40343 + t40347 + t40350 - F::cast_from(0.5185185185185185185e-1_f64) * t54631 + F::cast_from(0.65740740740740740737e-1_f64) * t54633 + F::cast_from(0.77777777777777777775e-2_f64) * t54635 - F::cast_from(0.2111111111111111111e-1_f64) * t54637 + F::cast_from(0.11234567901234567901e0_f64) * t54639 - F::cast_from(0.49999999999999999998e-2_f64) * t56465 + F::cast_from(0.16666666666666666666e-2_f64) * t56469 - F::cast_from(0.19999999999999999999e-1_f64) * t54643 - F::cast_from(0.39999999999999999998e-1_f64) * t40351 - F::cast_from(0.49999999999999999998e-2_f64) * t40356 + F::cast_from(0.16666666666666666666e-2_f64) * t40360;
    (t56463, t56467, t56475)
}
