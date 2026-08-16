//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 939/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk939<F: Float>(t1581: F, t950: F, t2766: F, t2824: F, t2912: F, t2919: F, t4335: F, t4340: F, t4345: F, t4349: F, t4363: F, t4371: F, t4379: F, t4381: F, t4384: F, t4387: F, t4390: F, t4393: F) -> (F, F) {
    let t4454 = t1581 * t950;
    let t4471 = -F::cast_from(0.1294625e1_f64) * t4363 + F::cast_from(0.258925e1_f64) * t4371 + t2912 + F::cast_from(0.10064166666666666667e0_f64) * t2766 + F::cast_from(0.10064166666666666667e0_f64) * t4335 - F::cast_from(0.20128333333333333333e0_f64) * t4340 + F::cast_from(0.60385e0_f64) * t4345 - F::cast_from(0.301925e0_f64) * t4349 + F::cast_from(0.82524375e-1_f64) * t4379 + F::cast_from(0.16504875e0_f64) * t4381 + t2919 + F::cast_from(0.5519e-1_f64) * t2824 + F::cast_from(0.5519e-1_f64) * t4384 - F::cast_from(0.27595e-1_f64) * t4387 + F::cast_from(0.16557e0_f64) * t4390 - F::cast_from(0.82785e-1_f64) * t4393;
    (t4454, t4471)
}
