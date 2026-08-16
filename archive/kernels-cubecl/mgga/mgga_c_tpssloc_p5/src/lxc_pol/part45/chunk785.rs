//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 785/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk785<F: Float>(t23519: F, t23520: F, t1940: F, t3046: F, t354: F, t1046: F, t1935: F, t23489: F, t23495: F, t23500: F, t23504: F, t23510: F, t23515: F, t3057: F, t3064: F, t6723: F, t6730: F, t6735: F, t6742: F, t6747: F, t6765: F) -> F {
    let t23521 = t23519 * t23520;
    let t23528 = t1940 * t3046;
    let t23529 = t354 * t23528;
    let t23532 = F::cast_from(0.20186378047070195428e-3_f64) * t23489 * t6747 - F::cast_from(0.20186378047070195428e-3_f64) * t6730 * t6735 - F::cast_from(0.10093189023535097714e-3_f64) * t1935 * t23495 + F::cast_from(0.16149102437656156342e-2_f64) * t6723 * t6735 + t23500 / F::cast_from(1152.0_f64) + F::cast_from(0.10093189023535097714e-3_f64) * t6742 * t23504 + F::cast_from(0.20186378047070195428e-3_f64) * t23510 * t23515 - F::cast_from(0.10093189023535097714e-3_f64) * t23510 * t23521 + t6765 * t3057 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t6765 * t3064 - t23529 * t1046 / F::cast_from(216.0_f64);
    t23532
}
