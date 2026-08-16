//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 800/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk800<F: Float>(t1561: F, t923: F, t1569: F, t931: F, t2766: F, t2824: F, t2868: F, t2875: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F, F, F) {
    let t4411 = t1561 * t923;
    let t4416 = t1569 * t931;
    let t4433 = -F::cast_from(0.17648625e1_f64) * t4363 + F::cast_from(0.3529725e1_f64) * t4371 + t2868 + F::cast_from(0.17215833333333333333e0_f64) * t2766 + F::cast_from(0.17215833333333333333e0_f64) * t4335 - F::cast_from(0.34431666666666666667e0_f64) * t4340 + F::cast_from(0.103295e1_f64) * t4345 - F::cast_from(0.516475e0_f64) * t4349 + F::cast_from(0.31558125e0_f64) * t4379 + F::cast_from(0.6311625e0_f64) * t4381 + t2875 + F::cast_from(0.69463333333333333333e-1_f64) * t2824 + F::cast_from(0.69463333333333333333e-1_f64) * t4384 - F::cast_from(0.34731666666666666667e-1_f64) * t4387 + F::cast_from(0.20839e0_f64) * t4390 - F::cast_from(0.104195e0_f64) * t4393;
    (t4411, t4416, t4433)
}
