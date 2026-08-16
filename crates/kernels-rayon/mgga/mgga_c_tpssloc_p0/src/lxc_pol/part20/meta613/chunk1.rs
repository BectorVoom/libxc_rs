//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2202/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2202(t2311: f64, t671: f64, t11968: f64, t1266: f64, t12724: f64, t12728: f64, t12835: f64, t12841: f64, t1442: f64, t1459: f64, t15857: f64, t1774: f64, t2312: f64, t3652: f64, t4026: f64, t4034: f64, t4037: f64, t45590: f64, t510: f64, t5107: f64, t650: f64, t9347: f64, t9348: f64, t9351: f64) -> (f64, f64) {
    let t45602 = t2311 * t671;
    let t45616 = -t11968 * t1442 - 3.0_f64 * t1266 * t12724 - 6.0_f64 * t1266 * t12728 - 6.0_f64 * t12835 * t4034 - 6.0_f64 * t12841 * t4034 - 6.0_f64 * t1459 * t45602 - 3.0_f64 * t15857 * t650 - t1774 * t9347 - 6.0_f64 * t1774 * t9351 - 3.0_f64 * t2312 * t5107 - 3.0_f64 * t3652 * t4026 - 6.0_f64 * t4037 * t9348 - 6.0_f64 * t45590 * t510;
    (t45602, t45616)
}
