//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2291/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2291<F: Float>(t13797: F, t17152: F, t17161: F, t17920: F, t18016: F, t18025: F, t1920: F, t1933: F, t1934: F, t23419: F, t25585: F, t25601: F, t25609: F, t378: F, t4509: F, t5842: F, t5904: F, t6735: F, t6758: F, t7578: F, t83016: F, t83080: F, t88566: F, t88569: F) -> F {
    let t99760 = -t23419 * t18025 / F::cast_from(576.0_f64) + t83016 * t18016 / F::cast_from(576.0_f64) + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t23419 * t17920 - t5904 * t6758 * t378 / F::cast_from(288.0_f64) - t88566 + t88569 + t83080 + t1920 * t4509 * t17161 / F::cast_from(108.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t1920 * t13797 * t17152 - F::cast_from(0.20186378047070195428e-3_f64) * t25601 * t25609 - F::cast_from(0.10093189023535097714e-3_f64) * t1933 * t1934 * t5842 * t6735 + F::cast_from(0.16149102437656156342e-2_f64) * t25585 * t7578;
    t99760
}
