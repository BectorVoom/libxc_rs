//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2202/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2202<F: Float>(t2311: F, t671: F, t11968: F, t1266: F, t12724: F, t12728: F, t12835: F, t12841: F, t1442: F, t1459: F, t15857: F, t1774: F, t2312: F, t3652: F, t4026: F, t4034: F, t4037: F, t45590: F, t510: F, t5107: F, t650: F, t9347: F, t9348: F, t9351: F) -> (F, F) {
    let t45602 = t2311 * t671;
    let t45616 = -t11968 * t1442 - F::cast_from(3.0_f64) * t1266 * t12724 - F::cast_from(6.0_f64) * t1266 * t12728 - F::cast_from(6.0_f64) * t12835 * t4034 - F::cast_from(6.0_f64) * t12841 * t4034 - F::cast_from(6.0_f64) * t1459 * t45602 - F::cast_from(3.0_f64) * t15857 * t650 - t1774 * t9347 - F::cast_from(6.0_f64) * t1774 * t9351 - F::cast_from(3.0_f64) * t2312 * t5107 - F::cast_from(3.0_f64) * t3652 * t4026 - F::cast_from(6.0_f64) * t4037 * t9348 - F::cast_from(6.0_f64) * t45590 * t510;
    (t45602, t45616)
}
