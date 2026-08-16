//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2599/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2599<F: Float>(t1734: F, t3507: F, t11721: F, t3493: F, t4978: F, t11786: F, t5005: F, t15730: F, t3536: F, t15594: F, t3523: F, t11678: F, t11684: F, t11805: F, t11809: F, t1215: F, t15569: F, t15659: F, t15660: F, t15761: F, t1653: F, t2244: F, t2250: F, t3247: F, t3490: F, t3578: F, t45197: F, t5024: F, t52687: F) -> (F, F, F, F) {
    let t52696 = t1734 * t3507;
    let t52704 = t1734 * t11721;
    let t52709 = t4978 * t3493;
    let t52725 = t5005 * t11786;
    let t52731 = t3536 * t15730;
    let t52732 = t52731 / F::cast_from(4608.0_f64);
    let t52733 = t15594 * t3523;
    let t52737 = t15569 * t11684 / F::cast_from(288.0_f64) - t45197 * t3578 * t52704 * t52687 / F::cast_from(256.0_f64) - t11678 * t3578 * t1653 * t52709 / F::cast_from(768.0_f64) - t11678 * t3578 * t15659 * t15660 * t2250 / F::cast_from(768.0_f64) - t11678 * t3578 * t15659 * t1215 * t3247 * t2244 / F::cast_from(384.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t52725 + t5024 * t11805 / F::cast_from(864.0_f64) + t5024 * t11809 / F::cast_from(144.0_f64) - t52732 - t52733 / F::cast_from(1152.0_f64) - t3490 * t15761 / F::cast_from(1536.0_f64);
    (t52696, t52704, t52709, t52737)
}
