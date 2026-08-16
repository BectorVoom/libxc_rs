//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2599/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2599(t1734: f64, t3507: f64, t11721: f64, t3493: f64, t4978: f64, t11786: f64, t5005: f64, t15730: f64, t3536: f64, t15594: f64, t3523: f64, t11678: f64, t11684: f64, t11805: f64, t11809: f64, t1215: f64, t15569: f64, t15659: f64, t15660: f64, t15761: f64, t1653: f64, t2244: f64, t2250: f64, t3247: f64, t3490: f64, t3578: f64, t45197: f64, t5024: f64, t52687: f64) -> (f64, f64, f64, f64) {
    let t52696 = t1734 * t3507;
    let t52704 = t1734 * t11721;
    let t52709 = t4978 * t3493;
    let t52725 = t5005 * t11786;
    let t52731 = t3536 * t15730;
    let t52732 = t52731 / 4608.0_f64;
    let t52733 = t15594 * t3523;
    let t52737 = t15569 * t11684 / 288.0_f64 - t45197 * t3578 * t52704 * t52687 / 256.0_f64 - t11678 * t3578 * t1653 * t52709 / 768.0_f64 - t11678 * t3578 * t15659 * t15660 * t2250 / 768.0_f64 - t11678 * t3578 * t15659 * t1215 * t3247 * t2244 / 384.0_f64 + 5.0_f64 / 6912.0_f64 * t52725 + t5024 * t11805 / 864.0_f64 + t5024 * t11809 / 144.0_f64 - t52732 - t52733 / 1152.0_f64 - t3490 * t15761 / 1536.0_f64;
    (t52696, t52704, t52709, t52737)
}
