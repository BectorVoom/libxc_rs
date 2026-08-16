//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1983/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1983<F: Float>(t3700: F, t2751: F, t10108: F, t257: F, t3639: F, t11604: F, t496: F, t111: F, t3931: F, t12723: F, t1406: F, t9238: F) -> (F, F, F, F, F, F, F, F) {
    let t40610 = t3700 * t3700;
    let t40611 = F::cast_from(1.0_f64) / t40610;
    let t40771 = t2751 * t2751;
    let t40772 = F::cast_from(1.0_f64) / t40771;
    let t40889 = F::cast_from(1.0_f64) / t10108 / t257;
    let t43705 = t3639 * t3639;
    let t43706 = F::cast_from(1.0_f64) / t43705;
    let t45349 = F::cast_from(1.0_f64) / t11604 / t496;
    let t45560 = t3931 * t111;
    let t45632 = t12723 * t111;
    let t45844 = t1406 * t9238;
    (t40611, t40772, t40889, t43706, t45349, t45560, t45632, t45844)
}
