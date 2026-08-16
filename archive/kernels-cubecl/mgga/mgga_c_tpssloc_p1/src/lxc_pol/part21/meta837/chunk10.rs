//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2988/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2988<F: Float>(t3070: F, t43198: F, t5908: F, t10937: F, t18041: F, t1041: F, t13969: F, t17636: F, t10408: F, t10413: F, t10952: F, t13528: F, t14143: F, t14147: F, t14184: F, t14489: F, t1616: F, t1622: F, t17718: F, t17738: F, t2776: F, t2960: F, t3039: F, t3071: F, t43358: F, t4582: F, t4593: F, t4644: F, t48432: F, t50047: F, t50056: F, t5878: F, t5909: F) -> F {
    let t62494 = t3070 * t43198 * t5908;
    let t62499 = t10937 * t18041;
    let t62510 = t1041 * t13969 * t17636;
    let t62512 = -t4644 * t14143 / F::cast_from(576.0_f64) - t4644 * t14147 / F::cast_from(1152.0_f64) - t2960 * t17738 / F::cast_from(54.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t4644 * t14184 + F::cast_from(5.0_f64) / F::cast_from(3456.0_f64) * t3070 * t10408 * t1616 * t13528 + t50047 / F::cast_from(2592.0_f64) + t10413 * t3071 * t5878 * t2776 / F::cast_from(2304.0_f64) - t62494 / F::cast_from(10368.0_f64) + t50056 / F::cast_from(3456.0_f64) + F::cast_from(19.0_f64) / F::cast_from(1296.0_f64) * t43358 * t5909 - t62499 / F::cast_from(324.0_f64) + t48432 * t1622 / F::cast_from(2304.0_f64) - t10952 * t17718 / F::cast_from(1536.0_f64) - t3039 * t4582 * t4593 * t14489 / F::cast_from(1536.0_f64) - t62510 / F::cast_from(1728.0_f64);
    t62512
}
