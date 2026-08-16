//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2680/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2680<F: Float>(t1315: F, t210: F, t214: F, t40343: F, t40347: F, t40350: F, t54631: F, t54633: F, t54638: F, t54639: F, t54644: F, t56465: F, t56469: F, t74355: F) -> F {
    let t74699 = -t40343 + t40347 + t40350 - F::cast_from(0.38888888888888888888e-1_f64) * t54631 + F::cast_from(0.98611111111111111109e-1_f64) * t54633 - t54638 + F::cast_from(0.16851851851851851851e0_f64) * t54639 - F::cast_from(0.14999999999999999999e-1_f64) * t56465 + F::cast_from(0.49999999999999999998e-2_f64) * t56469 - t54644 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t210 * t214 * t74355;
    t74699
}
