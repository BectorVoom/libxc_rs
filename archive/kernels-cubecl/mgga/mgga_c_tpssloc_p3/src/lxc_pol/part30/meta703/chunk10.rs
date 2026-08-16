//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2295/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2295<F: Float>(t7583: F, t88383: F, t25650: F, t3030: F, t88449: F, t1015: F, t17714: F, t17959: F, t23489: F, t23537: F, t25652: F, t25655: F, t25660: F, t25661: F, t28566: F, t28587: F, t360: F, t5866: F, t6730: F, t6742: F, t6744: F, t68: F, t83157: F, t88290: F, t88407: F, t88723: F) -> F {
    let t99834 = t88383 * t7583;
    let t99848 = t25650 * t88449 * t3030;
    let t99855 = F::cast_from(0.10093189023535097714e-3_f64) * t23489 * t28587 + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t6744 * t17959 * t68 * t360 - F::cast_from(0.20186378047070195428e-3_f64) * t99834 + F::cast_from(0.20186378047070195428e-3_f64) * t88290 * t7583 - t83157 / F::cast_from(1296.0_f64) - F::cast_from(0.10093189023535097714e-3_f64) * t6730 * t28566 - F::cast_from(0.10093189023535097714e-3_f64) * t25652 * t1015 * t5866 * t25660 - F::cast_from(0.20186378047070195428e-3_f64) * t88407 * t7583 + F::cast_from(0.40372756094140390856e-3_f64) * t99848 * t25655 - F::cast_from(0.20186378047070195428e-3_f64) * t99848 * t25661 + t23537 * t17714 / F::cast_from(768.0_f64) + t88723;
    t99855
}
