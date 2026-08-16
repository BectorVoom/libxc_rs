//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 839/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk839<F: Float>(t1053: F, t68: F, t134: F, t976: F, t3036: F, t67: F, t3215: F, t390: F, t1376: F, t1995: F, t246: F, t3700: F, t570: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t10163 = t1053 * t1053;
    let t10164 = F::cast_from(1.0_f64) / t10163;
    let t10165 = t68 * t10164;
    let t10189 = t134 * t976;
    let t10401 = t3036 * t67;
    let t11094 = F::cast_from(1.0_f64) / t3215 / t390;
    let t12019 = t1376 * t1376;
    let t12020 = F::cast_from(1.0_f64) / t12019;
    let t12021 = t68 * t12020;
    let t12418 = t1995 * t67;
    let t12419 = t12418 * t246;
    let t12461 = F::cast_from(1.0_f64) / t3700 / t570;
    (t10163, t10164, t10165, t10189, t10401, t11094, t12019, t12020, t12021, t12419, t12461)
}
